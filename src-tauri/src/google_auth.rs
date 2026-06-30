//! Purpose: validate Google OAuth and Calendar read-only access for Spike 2.
//! Responsibilities: desktop OAuth flow orchestration, secure token persistence,
//! access token refresh, and minimal upcoming-calendar retrieval.
//! Inputs: a Google OAuth desktop client ID, Tauri app paths, and user-triggered commands.
//! Outputs: authentication status, a persisted secure token set, and sanitized event metadata.
//! Non-responsibilities: Gmail, Drive, AI, local databases, or final product UX.

use std::{
    fs,
    io::{Read, Write},
    net::TcpListener,
    path::PathBuf,
    process::Command,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

const GOOGLE_AUTH_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";
const GOOGLE_CALENDAR_EVENTS_URL: &str =
    "https://www.googleapis.com/calendar/v3/calendars/primary/events";
const GOOGLE_CALENDAR_READONLY_SCOPE: &str =
    "https://www.googleapis.com/auth/calendar.readonly";
const GOOGLE_TOKEN_URL: &str = "https://oauth2.googleapis.com/token";
const CLIENT_CONFIG_FILE: &str = "google-oauth-client.json";
const ENCRYPTED_TOKEN_FILE: &str = "google-tokens.protected";

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GoogleAuthStatus {
    pub configured: bool,
    pub authenticated: bool,
    pub has_refresh_token: bool,
    pub client_id: Option<String>,
    pub has_client_secret: bool,
    pub expires_at_epoch_seconds: Option<u64>,
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
struct GoogleCalendarEvent {
    id: String,
    summary: Option<String>,
    status: Option<String>,
    start: GoogleCalendarDateValue,
    end: Option<GoogleCalendarDateValue>,
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
        expires_at_epoch_seconds: stored_tokens.and_then(|tokens| tokens.expires_at_epoch_seconds),
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
        scope = encode_uri_component(GOOGLE_CALENDAR_READONLY_SCOPE),
        challenge = encode_uri_component(challenge),
        state = encode_uri_component(state)
    )
}

fn encode_uri_component(value: &str) -> String {
    let mut encoded = String::with_capacity(value.len());

    for byte in value.bytes() {
        let keep = matches!(byte,
            b'A'..=b'Z'
                | b'a'..=b'z'
                | b'0'..=b'9'
                | b'-'
                | b'_'
                | b'.'
                | b'~');

        if keep {
            encoded.push(char::from(byte));
        } else {
            encoded.push('%');
            encoded.push_str(&format!("{byte:02X}"));
        }
    }

    encoded
}

fn parse_google_token_response(output: &str, refresh_token_fallback: Option<&str>) -> Result<StoredGoogleTokens, String> {
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

    Ok(StoredGoogleTokens {
        access_token: token_response.access_token,
        refresh_token,
        expires_at_epoch_seconds: token_response
            .expires_in
            .map(|seconds| now_epoch_seconds().saturating_add(seconds.saturating_sub(30))),
        scope: token_response
            .scope
            .unwrap_or_else(|| GOOGLE_CALENDAR_READONLY_SCOPE.to_string()),
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
        .ok_or_else(|| "Google OAuth client ID has not been configured yet.".to_string())?;
    let tokens = read_tokens_securely(app)?
        .ok_or_else(|| "Google authentication has not been completed yet.".to_string())?;

    if tokens.refresh_token.trim().is_empty() {
        return Err("Stored Google authentication is missing a refresh token.".to_string());
    }

    if tokens.scope != GOOGLE_CALENDAR_READONLY_SCOPE {
        return Err("Stored Google authentication does not match the Calendar read-only scope required by this spike.".to_string());
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
        .ok_or_else(|| "Google OAuth client ID has not been configured yet.".to_string())?;
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

    if tokens.scope != GOOGLE_CALENDAR_READONLY_SCOPE {
        return Err("Google returned scopes that differ from the Calendar read-only scope required by this spike.".to_string());
    }

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
  ('fields=' + [Uri]::EscapeDataString('items(id,summary,status,start,end)'))
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
        .map(|event| {
            let all_day = event.start.date_time.is_none();
            CalendarEventSummary {
                id: event.id,
                summary: event.summary.unwrap_or_else(|| "Untitled event".to_string()),
                start: event
                    .start
                    .date_time
                    .or(event.start.date)
                    .unwrap_or_else(|| "Unknown start".to_string()),
                end: event.end.and_then(|value| value.date_time.or(value.date)),
                all_day,
                status: event.status,
            }
        })
        .collect())
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

pub fn disconnect_google_impl(app: &AppHandle) -> Result<(), String> {
    delete_tokens_securely(app)
}
