//! Purpose: validate provider-neutral meeting-brief generation for Spike 4.
//! Responsibilities: local AI-provider configuration, minimal prompt construction,
//! provider-specific request dispatch, source-reference packaging, and validation-safe output shaping.
//! Inputs: selected-event context collected from Calendar, Gmail, and Drive plus local AI config.
//! Outputs: a generated meeting brief with confidence, summary text, and explicit source references.
//! Non-responsibilities: background scheduling, production ranking, full-content retrieval,
//! or any Google write operations.

use std::{fs, path::PathBuf, process::Command};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tauri::{AppHandle, Manager};

use crate::google_auth::MeetingContextCollection;

const AI_CONFIG_FILE: &str = "ai-provider-config.json";
const LEGACY_OPENAI_CONFIG_FILE: &str = "openai-brief-config.json";
const OPENAI_API_KEY_FILE: &str = "openai-api-key.protected";
const GEMINI_API_KEY_FILE: &str = "gemini-api-key.protected";
const OPENAI_RESPONSES_URL: &str = "https://api.openai.com/v1/responses";
const GEMINI_API_BASE_URL: &str = "https://generativelanguage.googleapis.com/v1beta/models";
const DEFAULT_OPENAI_MODEL: &str = "gpt-4.1";
const DEFAULT_GEMINI_MODEL: &str = "gemini-2.5-flash";

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum AiProviderKind {
    OpenAi,
    Gemini,
}

impl AiProviderKind {
    fn display_name(self) -> &'static str {
        match self {
            Self::OpenAi => "OpenAI",
            Self::Gemini => "Google Gemini",
        }
    }

    fn api_key_file_name(self) -> &'static str {
        match self {
            Self::OpenAi => OPENAI_API_KEY_FILE,
            Self::Gemini => GEMINI_API_KEY_FILE,
        }
    }

    fn default_model(self) -> &'static str {
        match self {
            Self::OpenAi => DEFAULT_OPENAI_MODEL,
            Self::Gemini => DEFAULT_GEMINI_MODEL,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProviderConfigStatus {
    pub provider: AiProviderKind,
    pub label: String,
    pub model: String,
    pub has_api_key: bool,
    pub configured: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AiProviderStatus {
    pub selected_provider: AiProviderKind,
    pub selected_provider_label: String,
    pub configured: bool,
    pub providers: Vec<ProviderConfigStatus>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SaveAiProviderConfigRequest {
    pub provider: AiProviderKind,
    pub api_key: Option<String>,
    pub model: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct SavedProviderConfig {
    model: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct SavedAiConfig {
    selected_provider: AiProviderKind,
    open_ai: SavedProviderConfig,
    gemini: SavedProviderConfig,
}

#[derive(Debug, Serialize, Deserialize)]
struct LegacyOpenAiConfig {
    model: String,
}

#[derive(Debug, Clone)]
struct ProviderRuntimeConfig {
    provider: AiProviderKind,
    model: String,
    api_key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BriefSourceReference {
    pub source_id: String,
    pub source_type: String,
    pub label: String,
    pub link: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GeneratedMeetingBrief {
    pub meeting_title: String,
    pub provider: AiProviderKind,
    pub provider_label: String,
    pub model: String,
    pub confidence_label: String,
    pub context_strength: String,
    pub brief_markdown: String,
    pub sources: Vec<BriefSourceReference>,
}

fn app_config_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_config_dir()
        .map_err(|error| format!("could not resolve app config directory: {error}"))?;

    fs::create_dir_all(&dir)
        .map_err(|error| format!("could not create app config directory: {error}"))?;

    Ok(dir)
}

fn ai_config_path(app: &AppHandle) -> Result<PathBuf, String> {
    Ok(app_config_dir(app)?.join(AI_CONFIG_FILE))
}

fn legacy_openai_config_path(app: &AppHandle) -> Result<PathBuf, String> {
    Ok(app_config_dir(app)?.join(LEGACY_OPENAI_CONFIG_FILE))
}

fn encrypted_api_key_path(app: &AppHandle, provider: AiProviderKind) -> Result<PathBuf, String> {
    Ok(app_config_dir(app)?.join(provider.api_key_file_name()))
}

fn default_saved_ai_config() -> SavedAiConfig {
    SavedAiConfig {
        selected_provider: AiProviderKind::OpenAi,
        open_ai: SavedProviderConfig {
            model: DEFAULT_OPENAI_MODEL.to_string(),
        },
        gemini: SavedProviderConfig {
            model: DEFAULT_GEMINI_MODEL.to_string(),
        },
    }
}

fn run_powershell(script: &str, envs: &[(&str, &str)]) -> Result<String, String> {
    let mut command = Command::new("powershell.exe");
    command.args(["-NoProfile", "-NonInteractive", "-Command", script]);

    for (key, value) in envs {
        command.env(key, value);
    }

    let output = command
        .output()
        .map_err(|error| format!("could not start PowerShell helper: {error}"))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let detail = if stderr.is_empty() { stdout } else { stderr };
        Err(if detail.is_empty() {
            "PowerShell helper failed.".to_string()
        } else {
            detail
        })
    }
}

fn read_saved_ai_config(app: &AppHandle) -> Result<SavedAiConfig, String> {
    let path = ai_config_path(app)?;
    if path.exists() {
        let contents = fs::read_to_string(&path)
            .map_err(|error| format!("could not read AI config: {error}"))?;
        return serde_json::from_str::<SavedAiConfig>(&contents)
            .map_err(|error| format!("could not parse AI config: {error}"));
    }

    let legacy_path = legacy_openai_config_path(app)?;
    if legacy_path.exists() {
        let contents = fs::read_to_string(&legacy_path)
            .map_err(|error| format!("could not read legacy AI config: {error}"))?;
        let legacy = serde_json::from_str::<LegacyOpenAiConfig>(&contents)
            .map_err(|error| format!("could not parse legacy AI config: {error}"))?;
        let mut config = default_saved_ai_config();
        if !legacy.model.trim().is_empty() {
            config.open_ai.model = legacy.model.trim().to_string();
        }
        return Ok(config);
    }

    Ok(default_saved_ai_config())
}

fn write_saved_ai_config(app: &AppHandle, config: &SavedAiConfig) -> Result<(), String> {
    let serialized = serde_json::to_string_pretty(config)
        .map_err(|error| format!("could not serialize AI config: {error}"))?;
    fs::write(ai_config_path(app)?, serialized)
        .map_err(|error| format!("could not save AI config: {error}"))?;
    Ok(())
}

fn provider_model_from_config(config: &SavedAiConfig, provider: AiProviderKind) -> String {
    match provider {
        AiProviderKind::OpenAi => config.open_ai.model.clone(),
        AiProviderKind::Gemini => config.gemini.model.clone(),
    }
}

fn set_provider_model(config: &mut SavedAiConfig, provider: AiProviderKind, model: &str) {
    let model_value = if model.trim().is_empty() {
        provider.default_model().to_string()
    } else {
        model.trim().to_string()
    };

    match provider {
        AiProviderKind::OpenAi => config.open_ai.model = model_value,
        AiProviderKind::Gemini => config.gemini.model = model_value,
    }
}

fn write_api_key_securely(
    app: &AppHandle,
    provider: AiProviderKind,
    api_key: &str,
) -> Result<(), String> {
    let api_key_path = encrypted_api_key_path(app, provider)?;
    let api_key_path_string = api_key_path.to_string_lossy().to_string();

    let script = r#"
Add-Type -AssemblyName System.Security
$plainBytes = [Text.Encoding]::UTF8.GetBytes($env:AI_PROVIDER_API_KEY_VALUE)
$protected = [System.Security.Cryptography.ProtectedData]::Protect(
  $plainBytes,
  $null,
  [System.Security.Cryptography.DataProtectionScope]::CurrentUser
)
[IO.File]::WriteAllText($env:AI_PROVIDER_API_KEY_PATH, [Convert]::ToBase64String($protected))
"#;

    run_powershell(
        script,
        &[
            ("AI_PROVIDER_API_KEY_VALUE", api_key),
            ("AI_PROVIDER_API_KEY_PATH", api_key_path_string.as_str()),
        ],
    )
    .map(|_| ())
}

fn read_api_key_securely(
    app: &AppHandle,
    provider: AiProviderKind,
) -> Result<Option<String>, String> {
    let api_key_path = encrypted_api_key_path(app, provider)?;
    if !api_key_path.exists() {
        return Ok(None);
    }

    let api_key_path_string = api_key_path.to_string_lossy().to_string();
    let script = r#"
Add-Type -AssemblyName System.Security
$protectedText = [IO.File]::ReadAllText($env:AI_PROVIDER_API_KEY_PATH)
$protectedBytes = [Convert]::FromBase64String($protectedText)
$plainBytes = [System.Security.Cryptography.ProtectedData]::Unprotect(
  $protectedBytes,
  $null,
  [System.Security.Cryptography.DataProtectionScope]::CurrentUser
)
[Text.Encoding]::UTF8.GetString($plainBytes)
"#;

    let output = run_powershell(
        script,
        &[("AI_PROVIDER_API_KEY_PATH", api_key_path_string.as_str())],
    )?;
    if output.trim().is_empty() {
        Ok(None)
    } else {
        Ok(Some(output))
    }
}

fn build_provider_status(
    config: &SavedAiConfig,
    provider: AiProviderKind,
    has_api_key: bool,
) -> ProviderConfigStatus {
    let model = provider_model_from_config(config, provider);
    ProviderConfigStatus {
        provider,
        label: provider.display_name().to_string(),
        model,
        has_api_key,
        configured: has_api_key,
    }
}

fn provider_runtime_config_from_saved(
    app: &AppHandle,
    config: &SavedAiConfig,
) -> Result<ProviderRuntimeConfig, String> {
    let provider = config.selected_provider;
    let api_key = read_api_key_securely(app, provider)?.ok_or_else(|| {
        format!(
            "{} is selected, but no local API key is configured yet.",
            provider.display_name()
        )
    })?;

    Ok(ProviderRuntimeConfig {
        provider,
        model: provider_model_from_config(config, provider),
        api_key,
    })
}

pub fn get_ai_provider_status_impl(app: &AppHandle) -> Result<AiProviderStatus, String> {
    let config = read_saved_ai_config(app)?;
    let openai_has_key = read_api_key_securely(app, AiProviderKind::OpenAi)?.is_some();
    let gemini_has_key = read_api_key_securely(app, AiProviderKind::Gemini)?.is_some();
    let providers = vec![
        build_provider_status(&config, AiProviderKind::OpenAi, openai_has_key),
        build_provider_status(&config, AiProviderKind::Gemini, gemini_has_key),
    ];

    let configured = providers
        .iter()
        .find(|provider| provider.provider == config.selected_provider)
        .map(|provider| provider.configured)
        .unwrap_or(false);

    Ok(AiProviderStatus {
        selected_provider: config.selected_provider,
        selected_provider_label: config.selected_provider.display_name().to_string(),
        configured,
        providers,
    })
}

pub fn save_ai_provider_config_impl(
    app: &AppHandle,
    request: SaveAiProviderConfigRequest,
) -> Result<AiProviderStatus, String> {
    let mut config = read_saved_ai_config(app)?;
    let model_value = request
        .model
        .unwrap_or_else(|| request.provider.default_model().to_string());
    set_provider_model(&mut config, request.provider, &model_value);
    config.selected_provider = request.provider;

    if let Some(api_key) = request.api_key {
        let trimmed_key = api_key.trim();
        if !trimmed_key.is_empty() {
            write_api_key_securely(app, request.provider, trimmed_key)?;
        } else if read_api_key_securely(app, request.provider)?.is_none() {
            return Err(format!(
                "A local {} API key is required before this provider can be used.",
                request.provider.display_name()
            ));
        }
    } else if read_api_key_securely(app, request.provider)?.is_none() {
        return Err(format!(
            "A local {} API key is required before this provider can be used.",
            request.provider.display_name()
        ));
    }

    write_saved_ai_config(app, &config)?;
    get_ai_provider_status_impl(app)
}

pub async fn generate_meeting_brief_impl(
    app: &AppHandle,
    context: MeetingContextCollection,
) -> Result<GeneratedMeetingBrief, String> {
    let config = read_saved_ai_config(app)?;
    let runtime_config = provider_runtime_config_from_saved(app, &config)?;
    let sources = build_source_references(&context);
    let confidence_label = derive_confidence_label(&context);
    let context_strength = derive_context_strength(&context);
    let prompt_payload = build_brief_prompt(&context, &sources, &confidence_label);
    let brief_markdown = generate_brief_with_provider(&runtime_config, &prompt_payload)?;

    Ok(GeneratedMeetingBrief {
        meeting_title: context.seed.summary,
        provider: runtime_config.provider,
        provider_label: runtime_config.provider.display_name().to_string(),
        model: runtime_config.model,
        confidence_label,
        context_strength,
        brief_markdown,
        sources,
    })
}

fn generate_brief_with_provider(
    runtime_config: &ProviderRuntimeConfig,
    prompt_payload: &str,
) -> Result<String, String> {
    match runtime_config.provider {
        AiProviderKind::OpenAi => generate_openai_brief(runtime_config, prompt_payload),
        AiProviderKind::Gemini => generate_gemini_brief(runtime_config, prompt_payload),
    }
}

fn generate_openai_brief(
    runtime_config: &ProviderRuntimeConfig,
    prompt_payload: &str,
) -> Result<String, String> {
    let request_body = json!({
        "model": runtime_config.model,
        "store": false,
        "max_output_tokens": 700,
        "instructions": shared_brief_instructions(),
        "input": prompt_payload
    });

    let response = invoke_openai_responses_api(&runtime_config.api_key, &request_body)?;
    extract_openai_output_text(&response)
}

fn generate_gemini_brief(
    runtime_config: &ProviderRuntimeConfig,
    prompt_payload: &str,
) -> Result<String, String> {
    let request_body = json!({
        "system_instruction": {
            "parts": [
                {
                    "text": shared_brief_instructions()
                }
            ]
        },
        "contents": [
            {
                "role": "user",
                "parts": [
                    {
                        "text": prompt_payload
                    }
                ]
            }
        ],
        "generationConfig": {
            "maxOutputTokens": 700
        }
    });

    let response = invoke_gemini_generate_content_api(
        &runtime_config.api_key,
        &runtime_config.model,
        &request_body,
    )?;
    extract_gemini_output_text(&response)
}

fn shared_brief_instructions() -> &'static str {
    "You are Meeting Prep Assistant. Generate a concise meeting-prep brief using only the provided context. Do not invent facts. If context is weak or missing, say so clearly. Use the exact source IDs provided. Return markdown with these sections in order: Meeting, Brief confidence / context strength, Key context, Recent related emails, Related documents, Open questions / preparation notes."
}

fn invoke_openai_responses_api(api_key: &str, request_body: &Value) -> Result<Value, String> {
    let body = serde_json::to_string(request_body)
        .map_err(|error| format!("could not serialize AI request: {error}"))?;

    let script = r#"
try {
  $response = Invoke-RestMethod -Method Post -Uri $env:OPENAI_RESPONSES_URL -Headers @{
    Authorization = 'Bearer ' + $env:AI_PROVIDER_API_KEY_VALUE
    'Content-Type' = 'application/json'
  } -Body $env:AI_PROVIDER_REQUEST_BODY
  $response | ConvertTo-Json -Compress -Depth 20
} catch {
  if ($_.Exception.Response) {
    $reader = New-Object IO.StreamReader($_.Exception.Response.GetResponseStream())
    $body = $reader.ReadToEnd()
    try {
      $json = $body | ConvertFrom-Json
      if ($json.error.message) {
        throw ('OpenAI request failed: ' + $json.error.message)
      }
    } catch {
    }
    throw 'OpenAI request failed.'
  }
  throw 'OpenAI request failed.'
}
"#;

    let output = run_powershell(
        script,
        &[
            ("OPENAI_RESPONSES_URL", OPENAI_RESPONSES_URL),
            ("AI_PROVIDER_API_KEY_VALUE", api_key),
            ("AI_PROVIDER_REQUEST_BODY", &body),
        ],
    )?;

    serde_json::from_str::<Value>(&output)
        .map_err(|error| format!("could not parse OpenAI response: {error}"))
}

fn invoke_gemini_generate_content_api(
    api_key: &str,
    model: &str,
    request_body: &Value,
) -> Result<Value, String> {
    let body = serde_json::to_string(request_body)
        .map_err(|error| format!("could not serialize AI request: {error}"))?;
    let endpoint = format!("{GEMINI_API_BASE_URL}/{model}:generateContent?key={api_key}");

    let script = r#"
try {
  $response = Invoke-RestMethod -Method Post -Uri $env:GEMINI_GENERATE_CONTENT_URL -Headers @{
    'Content-Type' = 'application/json'
  } -Body $env:AI_PROVIDER_REQUEST_BODY
  $response | ConvertTo-Json -Compress -Depth 20
} catch {
  if ($_.Exception.Response) {
    $reader = New-Object IO.StreamReader($_.Exception.Response.GetResponseStream())
    $body = $reader.ReadToEnd()
    try {
      $json = $body | ConvertFrom-Json
      if ($json.error.message) {
        throw ('Gemini request failed: ' + $json.error.message)
      }
    } catch {
    }
    throw 'Gemini request failed.'
  }
  throw 'Gemini request failed.'
}
"#;

    let output = run_powershell(
        script,
        &[
            ("GEMINI_GENERATE_CONTENT_URL", &endpoint),
            ("AI_PROVIDER_REQUEST_BODY", &body),
        ],
    )?;

    serde_json::from_str::<Value>(&output)
        .map_err(|error| format!("could not parse Gemini response: {error}"))
}

fn extract_openai_output_text(response: &Value) -> Result<String, String> {
    if let Some(output_text) = response.get("output_text").and_then(Value::as_str) {
        let trimmed = output_text.trim();
        if !trimmed.is_empty() {
            return Ok(trimmed.to_string());
        }
    }

    if let Some(output_items) = response.get("output").and_then(Value::as_array) {
        for item in output_items {
            if let Some(content_items) = item.get("content").and_then(Value::as_array) {
                for content in content_items {
                    if content.get("type").and_then(Value::as_str) == Some("output_text") {
                        if let Some(text) = content.get("text").and_then(Value::as_str) {
                            let trimmed = text.trim();
                            if !trimmed.is_empty() {
                                return Ok(trimmed.to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    Err("The OpenAI response did not contain any brief text.".to_string())
}

fn extract_gemini_output_text(response: &Value) -> Result<String, String> {
    let mut text_parts = Vec::new();

    if let Some(candidates) = response.get("candidates").and_then(Value::as_array) {
        for candidate in candidates {
            if let Some(parts) = candidate
                .get("content")
                .and_then(|content| content.get("parts"))
                .and_then(Value::as_array)
            {
                for part in parts {
                    if let Some(text) = part.get("text").and_then(Value::as_str) {
                        let trimmed = text.trim();
                        if !trimmed.is_empty() {
                            text_parts.push(trimmed.to_string());
                        }
                    }
                }
            }
        }
    }

    if text_parts.is_empty() {
        return Err("The Gemini response did not contain any brief text.".to_string());
    }

    Ok(text_parts.join("\n\n"))
}

fn build_source_references(context: &MeetingContextCollection) -> Vec<BriefSourceReference> {
    let mut sources = Vec::new();

    sources.push(BriefSourceReference {
        source_id: "Calendar-1".to_string(),
        source_type: "Calendar".to_string(),
        label: context.seed.summary.clone(),
        link: context.seed.html_link.clone(),
    });

    for (index, message) in context.gmail_results.iter().enumerate() {
        sources.push(BriefSourceReference {
            source_id: format!("Gmail-{}", index + 1),
            source_type: "Gmail".to_string(),
            label: message.subject.clone(),
            link: Some(message.source_link.clone()),
        });
    }

    for (index, file) in context.drive_results.iter().enumerate() {
        sources.push(BriefSourceReference {
            source_id: format!("Drive-{}", index + 1),
            source_type: "Drive".to_string(),
            label: file.name.clone(),
            link: file.web_view_link.clone(),
        });
    }

    sources
}

fn derive_confidence_label(context: &MeetingContextCollection) -> String {
    let evidence_count = context.gmail_results.len() + context.drive_results.len();
    match evidence_count {
        0 => "Low".to_string(),
        1..=2 => "Medium".to_string(),
        _ => "High".to_string(),
    }
}

fn derive_context_strength(context: &MeetingContextCollection) -> String {
    let evidence_count = context.gmail_results.len() + context.drive_results.len();
    if evidence_count == 0 {
        "Sparse context".to_string()
    } else if evidence_count <= 2 {
        "Moderate context".to_string()
    } else {
        "Strong context".to_string()
    }
}

fn build_brief_prompt(
    context: &MeetingContextCollection,
    sources: &[BriefSourceReference],
    confidence_label: &str,
) -> String {
    let gmail_lines = if context.gmail_results.is_empty() {
        "No related Gmail messages were found.".to_string()
    } else {
        context
            .gmail_results
            .iter()
            .enumerate()
            .map(|(index, message)| {
                format!(
                    "[Gmail-{index}] subject: {subject}; from: {from}; date: {date}; snippet: {snippet}",
                    index = index + 1,
                    subject = message.subject,
                    from = message.from.clone().unwrap_or_else(|| "Unknown sender".to_string()),
                    date = message.date.clone().unwrap_or_else(|| "Unknown date".to_string()),
                    snippet = message
                        .snippet
                        .clone()
                        .unwrap_or_else(|| "No snippet available".to_string())
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    };

    let drive_lines = if context.drive_results.is_empty() {
        "No related Drive documents were found.".to_string()
    } else {
        context
            .drive_results
            .iter()
            .enumerate()
            .map(|(index, file)| {
                format!(
                    "[Drive-{index}] name: {name}; type: {mime}; modified: {modified}",
                    index = index + 1,
                    name = file.name,
                    mime = file
                        .mime_type
                        .clone()
                        .unwrap_or_else(|| "Unknown type".to_string()),
                    modified = file
                        .modified_time
                        .clone()
                        .unwrap_or_else(|| "Unknown modified time".to_string())
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    };

    let source_lines = sources
        .iter()
        .map(|source| format!("[{}] {}: {}", source.source_id, source.source_type, source.label))
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "Selected meeting context:\n\
         [Calendar-1] title: {title}\n\
         start: {start}\n\
         end: {end}\n\
         status: {status}\n\
         attendees: {attendees}\n\
         description preview: {description}\n\
         suggested confidence baseline: {confidence}\n\n\
         Gmail context:\n{gmail}\n\n\
         Drive context:\n{drive}\n\n\
         Available sources:\n{sources}\n\n\
         Rules:\n\
         - Use only the context above.\n\
         - Do not guess facts that are not supported.\n\
         - If context is weak, say that clearly.\n\
         - Cite factual bullets with one or more source IDs in square brackets.\n\
         - Keep the brief concise and preparation-oriented.\n",
        title = context.seed.summary,
        start = context.seed.start,
        end = context
            .seed
            .end
            .clone()
            .unwrap_or_else(|| "Unknown".to_string()),
        status = context
            .seed
            .status
            .clone()
            .unwrap_or_else(|| "Unknown".to_string()),
        attendees = if context.seed.attendee_emails.is_empty() {
            "None exposed".to_string()
        } else {
            context.seed.attendee_emails.join(", ")
        },
        description = context
            .seed
            .description_preview
            .clone()
            .unwrap_or_else(|| "No description available".to_string()),
        confidence = confidence_label,
        gmail = gmail_lines,
        drive = drive_lines,
        sources = source_lines
    )
}
