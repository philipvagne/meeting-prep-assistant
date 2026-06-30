//! Purpose: validate Google OAuth and Google Workspace context collection for Spikes 2 and 3.
//! Responsibilities: desktop OAuth flow orchestration, secure token persistence,
//! access token refresh, Calendar event retrieval, and minimal Gmail / Drive context lookup.
//! Inputs: Google OAuth desktop client configuration, Tauri app paths, and user-triggered commands.
//! Outputs: authentication status, securely stored tokens, Calendar event metadata,
//! and validation-safe Gmail / Drive source metadata.
//! Non-responsibilities: AI generation, final meeting briefs, background monitoring,
//! or production-ready context ranking.

use std::{
    collections::HashSet,
    fs,
    io::{Read, Write},
    net::TcpListener,
    path::PathBuf,
    process::Command,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::{AppHandle, Manager};

const GOOGLE_AUTH_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";
const GOOGLE_TOKEN_URL: &str = "https://oauth2.googleapis.com/token";
const GOOGLE_CALENDAR_EVENTS_URL: &str =
    "https://www.googleapis.com/calendar/v3/calendars/primary/events";
const GOOGLE_CALENDAR_EVENT_URL: &str =
    "https://www.googleapis.com/calendar/v3/calendars/primary/events";
const GOOGLE_GMAIL_MESSAGES_URL: &str = "https://gmail.googleapis.com/gmail/v1/users/me/messages";
const GOOGLE_DRIVE_FILES_URL: &str = "https://www.googleapis.com/drive/v3/files";

const GOOGLE_CALENDAR_READONLY_SCOPE: &str =
    "https://www.googleapis.com/auth/calendar.readonly";
const GOOGLE_GMAIL_READONLY_SCOPE: &str = "https://www.googleapis.com/auth/gmail.readonly";
const GOOGLE_DRIVE_READONLY_SCOPE: &str = "https://www.googleapis.com/auth/drive.readonly";

const CLIENT_CONFIG_FILE: &str = "google-oauth-client.json";
const ENCRYPTED_TOKEN_FILE: &str = "google-tokens.protected";
const MAX_CALENDAR_EVENTS: usize = 10;
const MAX_GMAIL_RESULTS: usize = 6;
const MAX_DRIVE_RESULTS: usize = 6;
const MAX_KEYWORDS: usize = 6;
const MAX_ATTENDEE_EMAILS: usize = 4;

const KEYWORD_STOPWORDS: &[&str] = &[
    "about",
    "after",
    "agenda",
    "before",
    "between",
    "call",
    "check",
    "discussion",
    "followup",
    "follow",
    "from",
    "have",
    "into",
    "kickoff",
    "meeting",
    "notes",
    "project",
    "review",
    "status",
    "sync",
    "team",
    "that",
    "their",
    "there",
    "these",
    "this",
    "update",
    "with",
];

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GoogleAuthStatus {
    pub configured: bool,
    pub authenticated: bool,
    pub has_refresh_token: bool,
    pub client_id: Option<String>,
    pub has_client_secret: bool,
    pub expires_at_epoch_seconds: Option<u64>,
    pub granted_scopes: Vec<String>,
    pub has_required_scopes: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CalendarEventSummary {
    pub id: String,
    pub summary: String,
    pub start: String,
    pub end: Option<String>,
    pub all_day: bool,
    pub status: Option<String>,
    pub html_link: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CalendarSeedMetadata {
    pub id: String,
    pub summary: String,
    pub start: String,
    pub end: Option<String>,
    pub status: Option<String>,
    pub description_preview: Option<String>,
    pub attendee_emails: Vec<String>,
    pub html_link: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GmailContextResult {
    pub id: String,
    pub thread_id: String,
    pub subject: String,
    pub from: Option<String>,
    pub date: Option<String>,
    pub snippet: Option<String>,
    pub source_link: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DriveContextResult {
    pub id: String,
    pub name: String,
    pub mime_type: Option<String>,
    pub modified_time: Option<String>,
    pub web_view_link: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MeetingContextCollection {
    pub seed: CalendarSeedMetadata,
    pub gmail_results: Vec<GmailContextResult>,
    pub drive_results: Vec<DriveContextResult>,
    pub notes: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SavedGoogleClientConfig {
    pub client_id: String,
    #[serde(default)]
    pub client_secret: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct StoredGoogleTokens {
    access_token: String,
    refresh_token: String,
    expires_at_epoch_seconds: Option<u64>,
    scope: String,
}

#[derive(Debug, Deserialize)]
struct GoogleTokenResponse {
    access_token: String,
    expires_in: Option<u64>,
    refresh_token: Option<String>,
    scope: Option<String>,
    token_type: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GoogleCalendarEventsResponse {
    items: Vec<GoogleCalendarEvent>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GoogleCalendarEvent {
    id: String,
    summary: Option<String>,
    status: Option<String>,
    description: Option<String>,
    html_link: Option<String>,
    start: GoogleCalendarDateValue,
    end: Option<GoogleCalendarDateValue>,
    attendees: Option<Vec<GoogleCalendarAttendee>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GoogleCalendarAttendee {
    email: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GoogleCalendarDateValue {
    #[serde(rename = "dateTime")]
    date_time: Option<String>,
    date: Option<String>,
}

#[derive(Debug, Deserialize)]
struct PkceMaterial {
    state: String,
    verifier: String,
    challenge: String,
}

#[derive(Debug, Deserialize)]
struct GmailMessagesListResponse {
    messages: Option<Vec<GmailMessageListItem>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GmailMessageListItem {
    id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GmailMessageDetails {
    id: String,
    thread_id: String,
    snippet: Option<String>,
    payload: Option<GmailPayload>,
}

#[derive(Debug, Deserialize)]
struct GmailPayload {
    headers: Option<Vec<GmailHeader>>,
}

#[derive(Debug, Deserialize)]
struct GmailHeader {
    name: String,
    value: String,
}

#[derive(Debug, Deserialize)]
struct DriveFilesListResponse {
    files: Vec<DriveFileItem>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DriveFileItem {
    id: String,
    name: String,
    mime_type: Option<String>,
    modified_time: Option<String>,
    web_view_link: Option<String>,
}

fn required_scopes() -> [&'static str; 3] {
    [
        GOOGLE_CALENDAR_READONLY_SCOPE,
        GOOGLE_GMAIL_READONLY_SCOPE,
        GOOGLE_DRIVE_READONLY_SCOPE,
    ]
}

fn joined_required_scopes() -> String {
    required_scopes().join(" ")
}

fn normalize_scopes(scope_string: &str) -> Vec<String> {
    let mut scopes = scope_string
        .split_whitespace()
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
    scopes.sort();
    scopes.dedup();
    scopes
}

fn has_required_scopes(scope_string: &str) -> bool {
    let granted = normalize_scopes(scope_string);
    required_scopes()
        .into_iter()
        .all(|required| granted.iter().any(|scope| scope == required))
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

fn client_config_path(app: &AppHandle) -> Result<PathBuf, String> {
    Ok(app_config_dir(app)?.join(CLIENT_CONFIG_FILE))
}

fn encrypted_token_path(app: &AppHandle) -> Result<PathBuf, String> {
    Ok(app_config_dir(app)?.join(ENCRYPTED_TOKEN_FILE))
}

fn read_saved_client_config(app: &AppHandle) -> Result<Option<SavedGoogleClientConfig>, String> {
    let path = client_config_path(app)?;
    if !path.exists() {
        return Ok(None);
    }

    let contents =
        fs::read_to_string(&path).map_err(|error| format!("could not read OAuth client config: {error}"))?;
    let parsed = serde_json::from_str::<SavedGoogleClientConfig>(&contents)
        .map_err(|error| format!("could not parse OAuth client config: {error}"))?;

    Ok(Some(parsed))
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

fn generate_pkce_material() -> Result<PkceMaterial, String> {
    let script = r#"
$random = [System.Security.Cryptography.RandomNumberGenerator]::Create()
function New-Bytes([int]$count) {
  $bytes = New-Object byte[] $count
  $random.GetBytes($bytes)
  return $bytes
}
function To-Base64Url([byte[]]$bytes) {
  return [Convert]::ToBase64String($bytes).TrimEnd('=').Replace('+', '-').Replace('/', '_')
}
$verifier = To-Base64Url (New-Bytes 64)
$state = To-Base64Url (New-Bytes 24)
$sha = [System.Security.Cryptography.SHA256]::Create()
$challenge = To-Base64Url ($sha.ComputeHash([Text.Encoding]::ASCII.GetBytes($verifier)))
[pscustomobject]@{
  state = $state
  verifier = $verifier
  challenge = $challenge
} | ConvertTo-Json -Compress
"#;

    let output = run_powershell(script, &[])?;
    serde_json::from_str::<PkceMaterial>(&output)
        .map_err(|error| format!("could not parse PKCE helper output: {error}"))
}

fn open_system_browser(url: &str) -> Result<(), String> {
    let script = "Start-Process $env:GOOGLE_AUTH_URL";
    run_powershell(script, &[("GOOGLE_AUTH_URL", url)]).map(|_| ())
}

fn write_tokens_securely(app: &AppHandle, tokens: &StoredGoogleTokens) -> Result<(), String> {
    let token_path = encrypted_token_path(app)?;
    let token_path_string = token_path.to_string_lossy().to_string();
    let token_json = serde_json::to_string(tokens)
        .map_err(|error| format!("could not serialize secure token payload: {error}"))?;

    let script = r#"
Add-Type -AssemblyName System.Security
$plainBytes = [Text.Encoding]::UTF8.GetBytes($env:GOOGLE_TOKEN_JSON)
$protected = [System.Security.Cryptography.ProtectedData]::Protect(
  $plainBytes,
  $null,
  [System.Security.Cryptography.DataProtectionScope]::CurrentUser
)
[IO.File]::WriteAllText($env:GOOGLE_TOKEN_PATH, [Convert]::ToBase64String($protected))
"#;

    run_powershell(
        script,
        &[
            ("GOOGLE_TOKEN_JSON", token_json.as_str()),
            ("GOOGLE_TOKEN_PATH", token_path_string.as_str()),
        ],
    )
    .map(|_| ())
}

fn read_tokens_securely(app: &AppHandle) -> Result<Option<StoredGoogleTokens>, String> {
    let token_path = encrypted_token_path(app)?;
    if !token_path.exists() {
        return Ok(None);
    }

    let token_path_string = token_path.to_string_lossy().to_string();
    let script = r#"
Add-Type -AssemblyName System.Security
$protectedText = [IO.File]::ReadAllText($env:GOOGLE_TOKEN_PATH)
$protectedBytes = [Convert]::FromBase64String($protectedText)
$plainBytes = [System.Security.Cryptography.ProtectedData]::Unprotect(
  $protectedBytes,
  $null,
  [System.Security.Cryptography.DataProtectionScope]::CurrentUser
)
[Text.Encoding]::UTF8.GetString($plainBytes)
"#;

    let output = run_powershell(script, &[("GOOGLE_TOKEN_PATH", token_path_string.as_str())])?;
    let parsed = serde_json::from_str::<StoredGoogleTokens>(&output)
        .map_err(|error| format!("could not parse secure token payload: {error}"))?;

    Ok(Some(parsed))
}

fn delete_tokens_securely(app: &AppHandle) -> Result<(), String> {
    let token_path = encrypted_token_path(app)?;
    if token_path.exists() {
        fs::remove_file(token_path).map_err(|error| format!("could not clear stored Google tokens: {error}"))?;
    }
    Ok(())
}

fn now_epoch_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn save_google_client_config_impl_internal(
    app: &AppHandle,
    client_id: &str,
    client_secret: &str,
) -> Result<(), String> {
    let trimmed_client_id = client_id.trim();
    let trimmed_client_secret = client_secret.trim();
    if trimmed_client_id.is_empty() {
        return Err("Google OAuth client ID is required.".to_string());
    }

    if trimmed_client_secret.is_empty() {
        return Err("Google OAuth client secret is required.".to_string());
    }

    let config = SavedGoogleClientConfig {
        client_id: trimmed_client_id.to_string(),
        client_secret: trimmed_client_secret.to_string(),
    };
    let serialized = serde_json::to_string_pretty(&config)
        .map_err(|error| format!("could not serialize OAuth client config: {error}"))?;

    fs::write(client_config_path(app)?, serialized)
        .map_err(|error| format!("could not save OAuth client config: {error}"))?;

    Ok(())
}

pub fn save_google_client_config_impl(
    app: &AppHandle,
    client_id: String,
    client_secret: String,
) -> Result<GoogleAuthStatus, String> {
    save_google_client_config_impl_internal(app, &client_id, &client_secret)?;
    get_google_auth_status_impl(app)
}

pub fn get_google_auth_status_impl(app: &AppHandle) -> Result<GoogleAuthStatus, String> {
    let client_config = read_saved_client_config(app)?;
    let stored_tokens = read_tokens_securely(app)?;
    let granted_scopes = stored_tokens
        .as_ref()
        .map(|tokens| normalize_scopes(&tokens.scope))
        .unwrap_or_default();
    let scope_string = stored_tokens
        .as_ref()
        .map(|tokens| tokens.scope.clone())
        .unwrap_or_default();
    let expires_at_epoch_seconds = stored_tokens
        .as_ref()
        .and_then(|tokens| tokens.expires_at_epoch_seconds);

    Ok(GoogleAuthStatus {
        configured: client_config.is_some(),
        authenticated: stored_tokens.is_some(),
        has_refresh_token: stored_tokens
            .as_ref()
            .map(|tokens| !tokens.refresh_token.is_empty())
            .unwrap_or(false),
        client_id: client_config.as_ref().map(|config| config.client_id.clone()),
        has_client_secret: client_config
            .as_ref()
            .map(|config| !config.client_secret.trim().is_empty())
            .unwrap_or(false),
        expires_at_epoch_seconds,
        granted_scopes,
        has_required_scopes: has_required_scopes(&scope_string),
    })
}

fn percent_decode(value: &str) -> Result<String, String> {
    let mut bytes = Vec::with_capacity(value.len());
    let mut chars = value.as_bytes().iter().copied();

    while let Some(byte) = chars.next() {
        match byte {
            b'+' => bytes.push(b' '),
            b'%' => {
                let high = chars
                    .next()
                    .ok_or_else(|| "OAuth callback contained an incomplete percent-encoding.".to_string())?;
                let low = chars
                    .next()
                    .ok_or_else(|| "OAuth callback contained an incomplete percent-encoding.".to_string())?;
                let value = (hex_value(high)? << 4) | hex_value(low)?;
                bytes.push(value);
            }
            _ => bytes.push(byte),
        }
    }

    String::from_utf8(bytes).map_err(|error| format!("OAuth callback contained invalid UTF-8: {error}"))
}

fn hex_value(byte: u8) -> Result<u8, String> {
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        b'A'..=b'F' => Ok(byte - b'A' + 10),
        _ => Err("OAuth callback contained invalid percent-encoding.".to_string()),
    }
}

fn wait_for_oauth_callback(listener: TcpListener, expected_state: &str) -> Result<String, String> {
    listener
        .set_nonblocking(false)
        .map_err(|error| format!("could not configure OAuth callback listener: {error}"))?;

    let (mut stream, _) = listener
        .accept()
        .map_err(|error| format!("could not receive OAuth callback: {error}"))?;

    stream
        .set_read_timeout(Some(Duration::from_secs(180)))
        .map_err(|error| format!("could not set OAuth callback timeout: {error}"))?;

    let mut buffer = [0_u8; 4096];
    let bytes_read = stream
        .read(&mut buffer)
        .map_err(|error| format!("could not read OAuth callback request: {error}"))?;

    let request = String::from_utf8_lossy(&buffer[..bytes_read]);
    let request_line = request
        .lines()
        .next()
        .ok_or_else(|| "OAuth callback request was empty.".to_string())?;
    let callback_target = request_line
        .split_whitespace()
        .nth(1)
        .ok_or_else(|| "OAuth callback request line was malformed.".to_string())?;

    let query = callback_target.split_once('?').map(|(_, query)| query).unwrap_or("");
    let mut code = None;
    let mut state = None;
    let mut oauth_error = None;

    for pair in query.split('&') {
        if pair.is_empty() {
            continue;
        }

        let (key, value) = pair.split_once('=').unwrap_or((pair, ""));
        let decoded_key = percent_decode(key)?;
        let decoded_value = percent_decode(value)?;

        match decoded_key.as_str() {
            "code" => code = Some(decoded_value),
            "state" => state = Some(decoded_value),
            "error" => oauth_error = Some(decoded_value),
            _ => {}
        }
    }

    let success = oauth_error.is_none() && state.as_deref() == Some(expected_state) && code.is_some();
    let response_body = if success {
        "<html><body><h1>Authentication complete</h1><p>You can close this window and return to Meeting Prep Assistant.</p></body></html>"
    } else {
        "<html><body><h1>Authentication failed</h1><p>You can close this window and return to Meeting Prep Assistant.</p></body></html>"
    };
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        response_body.len(),
        response_body
    );
    let _ = stream.write_all(response.as_bytes());
    let _ = stream.flush();

    if let Some(error) = oauth_error {
        return Err(format!("Google authentication was not completed: {error}"));
    }

    if state.as_deref() != Some(expected_state) {
        return Err("Google authentication state validation failed.".to_string());
    }

    code.ok_or_else(|| "Google authentication callback did not include an authorization code.".to_string())
}

fn build_google_auth_url(client_id: &str, redirect_uri: &str, state: &str, challenge: &str) -> String {
    format!(
        "{GOOGLE_AUTH_URL}?client_id={client_id}&redirect_uri={redirect_uri}&response_type=code&scope={scope}&access_type=offline&prompt=consent&code_challenge={challenge}&code_challenge_method=S256&state={state}",
        client_id = encode_uri_component(client_id),
        redirect_uri = encode_uri_component(redirect_uri),
        scope = encode_uri_component(&joined_required_scopes()),
        challenge = encode_uri_component(challenge),
        state = encode_uri_component(state)
    )
}

fn encode_uri_component(value: &str) -> String {
    let mut encoded = String::with_capacity(value.len());

    for byte in value.bytes() {
        let keep = matches!(
            byte,
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~'
        );

        if keep {
            encoded.push(char::from(byte));
        } else {
            encoded.push('%');
            encoded.push_str(&format!("{byte:02X}"));
        }
    }

    encoded
}

fn parse_google_token_response(
    output: &str,
    refresh_token_fallback: Option<&str>,
) -> Result<StoredGoogleTokens, String> {
    let token_response = serde_json::from_str::<GoogleTokenResponse>(output)
        .map_err(|error| format!("could not parse Google token response: {error}"))?;

    if token_response.token_type.as_deref().unwrap_or("Bearer") != "Bearer" {
        return Err("Google returned an unexpected token type.".to_string());
    }

    let refresh_token = token_response
        .refresh_token
        .or_else(|| refresh_token_fallback.map(ToOwned::to_owned))
        .unwrap_or_default();

    if refresh_token.is_empty() {
        return Err("Google did not return a refresh token. Re-consent with offline access is required.".to_string());
    }

    let scope = token_response
        .scope
        .unwrap_or_else(joined_required_scopes);

    if !has_required_scopes(&scope) {
        return Err("Google did not grant the required Calendar, Gmail, and Drive read-only scopes.".to_string());
    }

    Ok(StoredGoogleTokens {
        access_token: token_response.access_token,
        refresh_token,
        expires_at_epoch_seconds: token_response
            .expires_in
            .map(|seconds| now_epoch_seconds().saturating_add(seconds.saturating_sub(30))),
        scope,
    })
}

fn exchange_authorization_code(
    client_id: &str,
    client_secret: &str,
    redirect_uri: &str,
    code_verifier: &str,
    authorization_code: &str,
) -> Result<StoredGoogleTokens, String> {
    let script = r#"
try {
  $response = Invoke-RestMethod -Method Post -Uri $env:GOOGLE_TOKEN_URL -Body @{
    client_id = $env:GOOGLE_CLIENT_ID
    client_secret = $env:GOOGLE_CLIENT_SECRET
    code = $env:GOOGLE_AUTH_CODE
    code_verifier = $env:GOOGLE_CODE_VERIFIER
    grant_type = 'authorization_code'
    redirect_uri = $env:GOOGLE_REDIRECT_URI
  } -ContentType 'application/x-www-form-urlencoded'
  $response | ConvertTo-Json -Compress -Depth 5
} catch {
  if ($_.Exception.Response) {
    $reader = New-Object IO.StreamReader($_.Exception.Response.GetResponseStream())
    $body = $reader.ReadToEnd()
    throw $body
  }
  throw
}
"#;

    let output = run_powershell(
        script,
        &[
            ("GOOGLE_TOKEN_URL", GOOGLE_TOKEN_URL),
            ("GOOGLE_CLIENT_ID", client_id),
            ("GOOGLE_CLIENT_SECRET", client_secret),
            ("GOOGLE_AUTH_CODE", authorization_code),
            ("GOOGLE_CODE_VERIFIER", code_verifier),
            ("GOOGLE_REDIRECT_URI", redirect_uri),
        ],
    )?;

    parse_google_token_response(&output, None)
}

fn refresh_access_token(
    client_id: &str,
    client_secret: &str,
    refresh_token: &str,
) -> Result<StoredGoogleTokens, String> {
    let script = r#"
try {
  $response = Invoke-RestMethod -Method Post -Uri $env:GOOGLE_TOKEN_URL -Body @{
    client_id = $env:GOOGLE_CLIENT_ID
    client_secret = $env:GOOGLE_CLIENT_SECRET
    grant_type = 'refresh_token'
    refresh_token = $env:GOOGLE_REFRESH_TOKEN
  } -ContentType 'application/x-www-form-urlencoded'
  $response | ConvertTo-Json -Compress -Depth 5
} catch {
  if ($_.Exception.Response) {
    $reader = New-Object IO.StreamReader($_.Exception.Response.GetResponseStream())
    $body = $reader.ReadToEnd()
    throw $body
  }
  throw
}
"#;

    let output = run_powershell(
        script,
        &[
            ("GOOGLE_TOKEN_URL", GOOGLE_TOKEN_URL),
            ("GOOGLE_CLIENT_ID", client_id),
            ("GOOGLE_CLIENT_SECRET", client_secret),
            ("GOOGLE_REFRESH_TOKEN", refresh_token),
        ],
    )?;

    parse_google_token_response(&output, Some(refresh_token))
}

fn authenticated_tokens(app: &AppHandle) -> Result<(SavedGoogleClientConfig, StoredGoogleTokens), String> {
    let client_config = read_saved_client_config(app)?
        .ok_or_else(|| "Google OAuth client configuration has not been saved yet.".to_string())?;
    let tokens = read_tokens_securely(app)?
        .ok_or_else(|| "Google authentication has not been completed yet.".to_string())?;

    if tokens.refresh_token.trim().is_empty() {
        return Err("Stored Google authentication is missing a refresh token.".to_string());
    }

    if !has_required_scopes(&tokens.scope) {
        return Err(
            "Stored Google authentication does not include Calendar, Gmail, and Drive read-only scopes. Reconnect Google to upgrade access."
                .to_string(),
        );
    }

    if client_config.client_secret.trim().is_empty() {
        return Err("Google OAuth client secret has not been configured yet.".to_string());
    }

    if let Some(expires_at_epoch_seconds) = tokens.expires_at_epoch_seconds {
        if expires_at_epoch_seconds <= now_epoch_seconds() {
            let refreshed = refresh_access_token(
                &client_config.client_id,
                &client_config.client_secret,
                &tokens.refresh_token,
            )?;
            write_tokens_securely(app, &refreshed)?;
            return Ok((client_config, refreshed));
        }
    }

    Ok((client_config, tokens))
}

pub async fn connect_google_impl(app: &AppHandle) -> Result<GoogleAuthStatus, String> {
    let client_config = read_saved_client_config(app)?
        .ok_or_else(|| "Google OAuth client configuration has not been saved yet.".to_string())?;

    if client_config.client_secret.trim().is_empty() {
        return Err("Google OAuth client secret has not been configured yet.".to_string());
    }

    let pkce_material = generate_pkce_material()?;
    let listener = TcpListener::bind("127.0.0.1:0")
        .map_err(|error| format!("could not start the OAuth callback listener: {error}"))?;
    let callback_address = listener
        .local_addr()
        .map_err(|error| format!("could not determine the OAuth callback listener address: {error}"))?;
    let redirect_uri = format!("http://127.0.0.1:{}/callback", callback_address.port());
    let auth_url = build_google_auth_url(
        &client_config.client_id,
        &redirect_uri,
        &pkce_material.state,
        &pkce_material.challenge,
    );

    open_system_browser(&auth_url)?;

    let expected_state = pkce_material.state.clone();
    let authorization_code =
        tauri::async_runtime::spawn_blocking(move || wait_for_oauth_callback(listener, &expected_state))
            .await
            .map_err(|error| format!("Google authentication flow did not finish cleanly: {error}"))??;

    let tokens = exchange_authorization_code(
        &client_config.client_id,
        &client_config.client_secret,
        &redirect_uri,
        &pkce_material.verifier,
        &authorization_code,
    )?;

    write_tokens_securely(app, &tokens)?;
    get_google_auth_status_impl(app)
}

fn fetch_upcoming_calendar_events_once(access_token: &str) -> Result<Vec<CalendarEventSummary>, String> {
    let script = r#"
$timeMin = [DateTime]::UtcNow.ToString('yyyy-MM-ddTHH:mm:ssZ')
$query = @(
  'maxResults=10',
  'orderBy=startTime',
  'singleEvents=true',
  ('timeMin=' + [Uri]::EscapeDataString($timeMin)),
  ('fields=' + [Uri]::EscapeDataString('items(id,summary,status,htmlLink,start,end)'))
) -join '&'
$uri = $env:GOOGLE_CALENDAR_EVENTS_URL + '?' + $query
try {
  $response = Invoke-RestMethod -Method Get -Uri $uri -Headers @{
    Authorization = 'Bearer ' + $env:GOOGLE_ACCESS_TOKEN
  }
  $response | ConvertTo-Json -Compress -Depth 6
} catch {
  if ($_.Exception.Response -and $_.Exception.Response.StatusCode.value__ -eq 401) {
    throw 'UNAUTHORIZED'
  }
  if ($_.Exception.Response) {
    $reader = New-Object IO.StreamReader($_.Exception.Response.GetResponseStream())
    $body = $reader.ReadToEnd()
    throw $body
  }
  throw
}
"#;

    let output = run_powershell(
        script,
        &[
            ("GOOGLE_CALENDAR_EVENTS_URL", GOOGLE_CALENDAR_EVENTS_URL),
            ("GOOGLE_ACCESS_TOKEN", access_token),
        ],
    )?;

    let payload = serde_json::from_str::<GoogleCalendarEventsResponse>(&output)
        .map_err(|error| format!("could not parse Google Calendar response: {error}"))?;

    Ok(payload
        .items
        .into_iter()
        .take(MAX_CALENDAR_EVENTS)
        .map(calendar_event_summary_from_api)
        .collect())
}

fn calendar_event_summary_from_api(event: GoogleCalendarEvent) -> CalendarEventSummary {
    let all_day = event.start.date_time.is_none();
    CalendarEventSummary {
        id: event.id,
        summary: event.summary.unwrap_or_else(|| "Untitled event".to_string()),
        start: calendar_value_to_string(event.start),
        end: event.end.map(calendar_value_to_string),
        all_day,
        status: event.status,
        html_link: event.html_link,
    }
}

fn calendar_value_to_string(value: GoogleCalendarDateValue) -> String {
    value
        .date_time
        .or(value.date)
        .unwrap_or_else(|| "Unknown".to_string())
}

pub async fn fetch_upcoming_calendar_events_impl(app: &AppHandle) -> Result<Vec<CalendarEventSummary>, String> {
    let (client_config, tokens) = authenticated_tokens(app)?;

    match fetch_upcoming_calendar_events_once(&tokens.access_token) {
        Ok(events) => Ok(events),
        Err(error) if error.contains("UNAUTHORIZED") => {
            let refreshed = refresh_access_token(
                &client_config.client_id,
                &client_config.client_secret,
                &tokens.refresh_token,
            )?;
            write_tokens_securely(app, &refreshed)?;
            fetch_upcoming_calendar_events_once(&refreshed.access_token)
        }
        Err(error) => Err(error),
    }
}

fn fetch_calendar_event_details_once(
    access_token: &str,
    event_id: &str,
) -> Result<CalendarSeedMetadata, String> {
    let script = r#"
$fields = [Uri]::EscapeDataString('id,summary,status,description,htmlLink,start,end,attendees(email)')
$uri = $env:GOOGLE_CALENDAR_EVENT_URL + '/' + [Uri]::EscapeDataString($env:GOOGLE_EVENT_ID) + '?fields=' + $fields
try {
  $response = Invoke-RestMethod -Method Get -Uri $uri -Headers @{
    Authorization = 'Bearer ' + $env:GOOGLE_ACCESS_TOKEN
  }
  $response | ConvertTo-Json -Compress -Depth 6
} catch {
  if ($_.Exception.Response -and $_.Exception.Response.StatusCode.value__ -eq 401) {
    throw 'UNAUTHORIZED'
  }
  if ($_.Exception.Response) {
    $reader = New-Object IO.StreamReader($_.Exception.Response.GetResponseStream())
    $body = $reader.ReadToEnd()
    throw $body
  }
  throw
}
"#;

    let output = run_powershell(
        script,
        &[
            ("GOOGLE_CALENDAR_EVENT_URL", GOOGLE_CALENDAR_EVENT_URL),
            ("GOOGLE_ACCESS_TOKEN", access_token),
            ("GOOGLE_EVENT_ID", event_id),
        ],
    )?;

    let event = serde_json::from_str::<GoogleCalendarEvent>(&output)
        .map_err(|error| format!("could not parse selected Calendar event response: {error}"))?;

    Ok(calendar_seed_from_api(event))
}

fn calendar_seed_from_api(event: GoogleCalendarEvent) -> CalendarSeedMetadata {
    let attendee_emails = event
        .attendees
        .unwrap_or_default()
        .into_iter()
        .filter_map(|attendee| attendee.email)
        .take(MAX_ATTENDEE_EMAILS)
        .collect::<Vec<_>>();

    CalendarSeedMetadata {
        id: event.id,
        summary: event.summary.unwrap_or_else(|| "Untitled event".to_string()),
        start: calendar_value_to_string(event.start),
        end: event.end.map(calendar_value_to_string),
        status: event.status,
        description_preview: event.description.map(|description| trim_for_display(&description, 240)),
        attendee_emails,
        html_link: event.html_link,
    }
}

fn trim_for_display(value: &str, max_len: usize) -> String {
    let cleaned = value.split_whitespace().collect::<Vec<_>>().join(" ");
    if cleaned.chars().count() <= max_len {
        cleaned
    } else {
        let trimmed = cleaned.chars().take(max_len).collect::<String>();
        format!("{trimmed}...")
    }
}

fn extract_keywords(text: &str) -> Vec<String> {
    let mut seen = HashSet::new();
    text.split(|character: char| !character.is_alphanumeric())
        .filter_map(|token| {
            let lowered = token.trim().to_lowercase();
            let valid = lowered.len() >= 4
                && lowered.len() <= 24
                && !KEYWORD_STOPWORDS.iter().any(|stopword| *stopword == lowered)
                && lowered.chars().any(|character| character.is_alphabetic());
            if valid && seen.insert(lowered.clone()) {
                Some(lowered)
            } else {
                None
            }
        })
        .take(MAX_KEYWORDS)
        .collect()
}

fn escape_for_gmail_query(value: &str) -> String {
    value.replace('"', "")
}

fn build_gmail_queries(seed: &CalendarSeedMetadata) -> Vec<String> {
    let mut queries = Vec::new();

    let summary = seed.summary.trim();
    if !summary.is_empty() && summary != "Untitled event" {
        queries.push(format!("\"{}\"", escape_for_gmail_query(summary)));
    }

    for email in seed.attendee_emails.iter().take(MAX_ATTENDEE_EMAILS) {
        queries.push(email.clone());
    }

    if let Some(description_preview) = &seed.description_preview {
        let keywords = extract_keywords(description_preview);
        if !keywords.is_empty() {
            queries.push(keywords.join(" "));
        }
    }

    let title_keywords = extract_keywords(summary);
    if !title_keywords.is_empty() {
        queries.push(title_keywords.join(" "));
    }

    queries.truncate(4);
    queries
}

fn build_drive_query(seed: &CalendarSeedMetadata) -> Option<String> {
    let mut terms = extract_keywords(&seed.summary);
    if let Some(description_preview) = &seed.description_preview {
        for keyword in extract_keywords(description_preview) {
            if !terms.iter().any(|existing| existing == &keyword) {
                terms.push(keyword);
            }
        }
    }
    terms.truncate(4);

    if terms.is_empty() {
        return None;
    }

    let clauses = terms
        .into_iter()
        .map(|keyword| {
            let escaped = keyword.replace('\'', "\\'");
            format!("name contains '{escaped}' or fullText contains '{escaped}'")
        })
        .collect::<Vec<_>>();

    Some(format!("trashed = false and ({})", clauses.join(" or ")))
}

fn fetch_gmail_results_once(
    access_token: &str,
    seed: &CalendarSeedMetadata,
) -> Result<Vec<GmailContextResult>, String> {
    let queries = build_gmail_queries(seed);
    if queries.is_empty() {
        return Ok(Vec::new());
    }

    let mut results = Vec::new();
    let mut seen_ids = HashSet::new();

    for query in queries {
        let script = r#"
$fields = [Uri]::EscapeDataString('messages(id,threadId)')
$uri = $env:GOOGLE_GMAIL_MESSAGES_URL + '?maxResults=4&fields=' + $fields + '&q=' + [Uri]::EscapeDataString($env:GOOGLE_GMAIL_QUERY)
try {
  $response = Invoke-RestMethod -Method Get -Uri $uri -Headers @{
    Authorization = 'Bearer ' + $env:GOOGLE_ACCESS_TOKEN
  }
  $response | ConvertTo-Json -Compress -Depth 6
} catch {
  if ($_.Exception.Response -and $_.Exception.Response.StatusCode.value__ -eq 401) {
    throw 'UNAUTHORIZED'
  }
  if ($_.Exception.Response) {
    $reader = New-Object IO.StreamReader($_.Exception.Response.GetResponseStream())
    $body = $reader.ReadToEnd()
    throw $body
  }
  throw
}
"#;

        let output = run_powershell(
            script,
            &[
                ("GOOGLE_GMAIL_MESSAGES_URL", GOOGLE_GMAIL_MESSAGES_URL),
                ("GOOGLE_ACCESS_TOKEN", access_token),
                ("GOOGLE_GMAIL_QUERY", &query),
            ],
        )?;

        for message in parse_gmail_messages_list_output(&output)? {
            if results.len() >= MAX_GMAIL_RESULTS {
                return Ok(results);
            }

            if !seen_ids.insert(message.id.clone()) {
                continue;
            }

            if let Some(result) = fetch_gmail_message_details_once(access_token, &message.id)? {
                results.push(result);
            }
        }
    }

    Ok(results)
}

fn parse_gmail_messages_list_output(output: &str) -> Result<Vec<GmailMessageListItem>, String> {
    let trimmed = output.trim();
    if trimmed.is_empty() {
        return Ok(Vec::new());
    }

    let value = serde_json::from_str::<Value>(trimmed)
        .map_err(|error| format!("could not parse Gmail message search response: {error}"))?;

    match value {
        Value::Null => Ok(Vec::new()),
        Value::String(_) => Ok(Vec::new()),
        Value::Array(_) => Ok(Vec::new()),
        Value::Object(_) => {
            let response = serde_json::from_value::<GmailMessagesListResponse>(value)
                .map_err(|error| format!("could not parse Gmail message search response: {error}"))?;
            Ok(response.messages.unwrap_or_default())
        }
        _ => Ok(Vec::new()),
    }
}

fn fetch_gmail_message_details_once(
    access_token: &str,
    message_id: &str,
) -> Result<Option<GmailContextResult>, String> {
    let script = r#"
$fields = [Uri]::EscapeDataString('id,threadId,snippet,payload(headers)')
$uri = $env:GOOGLE_GMAIL_MESSAGES_URL + '/' + [Uri]::EscapeDataString($env:GOOGLE_MESSAGE_ID) + '?format=metadata&metadataHeaders=Subject&metadataHeaders=From&metadataHeaders=Date&fields=' + $fields
try {
  $response = Invoke-RestMethod -Method Get -Uri $uri -Headers @{
    Authorization = 'Bearer ' + $env:GOOGLE_ACCESS_TOKEN
  }
  $response | ConvertTo-Json -Compress -Depth 8
} catch {
  if ($_.Exception.Response -and $_.Exception.Response.StatusCode.value__ -eq 404) {
    return
  }
  if ($_.Exception.Response -and $_.Exception.Response.StatusCode.value__ -eq 401) {
    throw 'UNAUTHORIZED'
  }
  if ($_.Exception.Response) {
    $reader = New-Object IO.StreamReader($_.Exception.Response.GetResponseStream())
    $body = $reader.ReadToEnd()
    throw $body
  }
  throw
}
"#;

    let output = run_powershell(
        script,
        &[
            ("GOOGLE_GMAIL_MESSAGES_URL", GOOGLE_GMAIL_MESSAGES_URL),
            ("GOOGLE_ACCESS_TOKEN", access_token),
            ("GOOGLE_MESSAGE_ID", message_id),
        ],
    )?;

    if output.is_empty() {
        return Ok(None);
    }

    let Some(details) = parse_gmail_message_details_output(&output)? else {
        return Ok(None);
    };

    let headers = details
        .payload
        .and_then(|payload| payload.headers)
        .unwrap_or_default();

    let subject = header_value(&headers, "Subject").unwrap_or_else(|| "No subject".to_string());
    let from = header_value(&headers, "From");
    let date = header_value(&headers, "Date");

    Ok(Some(GmailContextResult {
        id: details.id,
        thread_id: details.thread_id.clone(),
        subject,
        from,
        date,
        snippet: details
            .snippet
            .map(|snippet| trim_for_display(&snippet, 180))
            .filter(|snippet| !snippet.is_empty()),
        source_link: format!("https://mail.google.com/mail/u/0/#all/{}", details.thread_id),
    }))
}

fn parse_gmail_message_details_output(output: &str) -> Result<Option<GmailMessageDetails>, String> {
    let trimmed = output.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }

    let value = serde_json::from_str::<Value>(trimmed)
        .map_err(|error| format!("could not parse Gmail message details response: {error}"))?;

    match value {
        Value::Null => Ok(None),
        Value::String(_) => Ok(None),
        Value::Array(_) => Ok(None),
        Value::Object(_) => {
            let details = serde_json::from_value::<GmailMessageDetails>(value)
                .map_err(|error| format!("could not parse Gmail message details response: {error}"))?;
            Ok(Some(details))
        }
        _ => Ok(None),
    }
}

fn header_value(headers: &[GmailHeader], name: &str) -> Option<String> {
    headers
        .iter()
        .find(|header| header.name.eq_ignore_ascii_case(name))
        .map(|header| header.value.clone())
}

fn fetch_drive_results_once(
    access_token: &str,
    seed: &CalendarSeedMetadata,
) -> Result<Vec<DriveContextResult>, String> {
    let Some(query) = build_drive_query(seed) else {
        return Ok(Vec::new());
    };

    let script = r#"
$fields = [Uri]::EscapeDataString('files(id,name,mimeType,modifiedTime,webViewLink)')
$uri = $env:GOOGLE_DRIVE_FILES_URL + '?pageSize=6&orderBy=modifiedTime desc&fields=' + $fields + '&q=' + [Uri]::EscapeDataString($env:GOOGLE_DRIVE_QUERY)
try {
  $response = Invoke-RestMethod -Method Get -Uri $uri -Headers @{
    Authorization = 'Bearer ' + $env:GOOGLE_ACCESS_TOKEN
  }
  $response | ConvertTo-Json -Compress -Depth 6
} catch {
  if ($_.Exception.Response -and $_.Exception.Response.StatusCode.value__ -eq 401) {
    throw 'UNAUTHORIZED'
  }
  if ($_.Exception.Response) {
    $reader = New-Object IO.StreamReader($_.Exception.Response.GetResponseStream())
    $body = $reader.ReadToEnd()
    throw $body
  }
  throw
}
"#;

    let output = run_powershell(
        script,
        &[
            ("GOOGLE_DRIVE_FILES_URL", GOOGLE_DRIVE_FILES_URL),
            ("GOOGLE_ACCESS_TOKEN", access_token),
            ("GOOGLE_DRIVE_QUERY", &query),
        ],
    )?;

    let response = serde_json::from_str::<DriveFilesListResponse>(&output)
        .map_err(|error| format!("could not parse Google Drive search response: {error}"))?;

    Ok(response
        .files
        .into_iter()
        .take(MAX_DRIVE_RESULTS)
        .map(|file| DriveContextResult {
            id: file.id,
            name: file.name,
            mime_type: file.mime_type,
            modified_time: file.modified_time,
            web_view_link: file.web_view_link,
        })
        .collect())
}

pub async fn collect_meeting_context_impl(
    app: &AppHandle,
    event_id: String,
) -> Result<MeetingContextCollection, String> {
    let (client_config, tokens) = authenticated_tokens(app)?;

    let seed = match fetch_calendar_event_details_once(&tokens.access_token, &event_id) {
        Ok(seed) => seed,
        Err(error) if error.contains("UNAUTHORIZED") => {
            let refreshed = refresh_access_token(
                &client_config.client_id,
                &client_config.client_secret,
                &tokens.refresh_token,
            )?;
            write_tokens_securely(app, &refreshed)?;
            return collect_meeting_context_after_refresh(&refreshed.access_token, &event_id);
        }
        Err(error) => return Err(error),
    };

    collect_context_from_seed(&tokens.access_token, seed)
}

fn collect_meeting_context_after_refresh(
    access_token: &str,
    event_id: &str,
) -> Result<MeetingContextCollection, String> {
    let seed = fetch_calendar_event_details_once(access_token, event_id)?;
    collect_context_from_seed(access_token, seed)
}

fn collect_context_from_seed(
    access_token: &str,
    seed: CalendarSeedMetadata,
) -> Result<MeetingContextCollection, String> {
    let gmail_results = fetch_gmail_results_once(access_token, &seed)?;
    let drive_results = fetch_drive_results_once(access_token, &seed)?;
    let mut notes = Vec::new();

    if gmail_results.is_empty() {
        notes.push("No related Gmail messages were found for the selected event.".to_string());
    }

    if drive_results.is_empty() {
        notes.push("No related Google Drive files were found for the selected event.".to_string());
    }

    if seed.attendee_emails.is_empty() {
        notes.push("The selected event did not expose attendee email addresses, so Gmail matching relied on title and description keywords only.".to_string());
    }

    if seed.description_preview.is_none() {
        notes.push("The selected event did not include a description, so keyword extraction relied on the meeting title only.".to_string());
    }

    Ok(MeetingContextCollection {
        seed,
        gmail_results,
        drive_results,
        notes,
    })
}

pub fn disconnect_google_impl(app: &AppHandle) -> Result<(), String> {
    delete_tokens_securely(app)
}
