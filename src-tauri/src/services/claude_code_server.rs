//! Localhost-only HTTP/1.1 hook receiver for Claude Code lifecycle events.
//!
//! Binds strictly to `127.0.0.1:<port>` using Tokio TCP networking with zero
//! additional HTTP framework dependencies. Always responds immediately with
//! `200 OK` and `{}` on valid events so Claude Code is never blocked.

use crate::domain::claude_code::{ClaudeHookPayload, CLAUDE_CODE_HOOK_PATH};
use crate::models::types::{AppState, EngineEvent};
use crate::services::claude_code_config::hook_endpoint_url;
use log::{debug, info, warn};
use tauri::Emitter;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::Sender;
use tokio::time::{timeout, Duration};

const MAX_HEADER_BYTES: usize = 16 * 1024;
const MAX_BODY_BYTES: usize = 8 * 1024 * 1024;
const CONNECTION_TIMEOUT: Duration = Duration::from_secs(2);

/// Spawns the localhost HTTP hook server for Claude Code.
///
/// Updates `AppState` with listening status (or bind error) and emits
/// `claude-code-status-changed` without crashing the application if the port
/// is unavailable.
pub fn start_claude_code_server(port: u16, tx: Sender<EngineEvent>, app_state: AppState) {
    tauri::async_runtime::spawn(async move {
        let addr = format!("127.0.0.1:{port}");
        match TcpListener::bind(&addr).await {
            Ok(listener) => {
                info!("HTTP hook listener started on {}", hook_endpoint_url(port));
                app_state.set_claude_server_status(port, true, None).await;
                let status = app_state.get_claude_code_status().await;
                let _ = app_state
                    .app_handle
                    .emit("claude-code-status-changed", &status);

                serve_listener(listener, tx).await;
            }
            Err(err) => {
                let msg = format!("Could not listen on {addr}: {err}");
                warn!("{msg}");
                app_state
                    .set_claude_server_status(port, false, Some(msg))
                    .await;
                let status = app_state.get_claude_code_status().await;
                let _ = app_state
                    .app_handle
                    .emit("claude-code-status-changed", &status);
            }
        }
    });
}

/// Runs the TCP accept loop on an already-bound listener.
pub async fn serve_listener(listener: TcpListener, tx: Sender<EngineEvent>) {
    loop {
        match listener.accept().await {
            Ok((stream, _peer)) => {
                let tx = tx.clone();
                tokio::spawn(async move {
                    if let Err(err) =
                        timeout(CONNECTION_TIMEOUT, handle_connection(stream, tx)).await
                    {
                        debug!("Hook HTTP connection timed out: {err}");
                    }
                });
            }
            Err(err) => {
                warn!("Accept error on hook listener: {err}");
                tokio::time::sleep(Duration::from_millis(200)).await;
            }
        }
    }
}

async fn handle_connection(mut stream: TcpStream, tx: Sender<EngineEvent>) {
    let response = match read_http_request(&mut stream).await {
        Ok(request) => route_request(&request, &tx),
        Err(ReadError::HeadersTooLarge) => http_json_response(
            431,
            "Request Header Fields Too Large",
            r#"{"error":"headers too large"}"#,
        ),
        Err(ReadError::PayloadTooLarge) => {
            http_json_response(413, "Payload Too Large", r#"{"error":"payload too large"}"#)
        }
        Err(ReadError::BadRequest) => {
            http_json_response(400, "Bad Request", r#"{"error":"bad request"}"#)
        }
        Err(ReadError::Io) => return,
    };

    let _ = stream.write_all(&response).await;
    let _ = stream.shutdown().await;
}

#[derive(Debug, PartialEq, Eq)]
struct ParsedRequest {
    method: String,
    path: String,
    body: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq)]
enum ReadError {
    HeadersTooLarge,
    PayloadTooLarge,
    BadRequest,
    Io,
}

#[derive(Debug, PartialEq, Eq)]
struct HeaderBlock {
    method: String,
    path: String,
    content_length: Option<usize>,
    is_chunked: bool,
    expect_continue: bool,
}

async fn read_http_request(stream: &mut TcpStream) -> Result<ParsedRequest, ReadError> {
    let mut buf = Vec::with_capacity(4096);
    let mut temp = [0u8; 8192];

    let header_end = loop {
        if let Some(pos) = find_header_end(&buf) {
            break pos;
        }
        if buf.len() >= MAX_HEADER_BYTES {
            return Err(ReadError::HeadersTooLarge);
        }
        let n = stream.read(&mut temp).await.map_err(|_| ReadError::Io)?;
        if n == 0 {
            return Err(ReadError::BadRequest);
        }
        buf.extend_from_slice(&temp[..n]);
    };

    let header_bytes = &buf[..header_end];
    let headers = parse_header_block(header_bytes)?;
    let mut remainder = buf[header_end + 4..].to_vec();

    if headers.expect_continue {
        let _ = stream.write_all(b"HTTP/1.1 100 Continue\r\n\r\n").await;
    }

    let body = if headers.is_chunked {
        read_chunked_body(stream, &mut remainder).await?
    } else if let Some(len) = headers.content_length {
        if len > MAX_BODY_BYTES {
            return Err(ReadError::PayloadTooLarge);
        }
        remainder.reserve(len.saturating_sub(remainder.len()));
        while remainder.len() < len {
            let n = stream.read(&mut temp).await.map_err(|_| ReadError::Io)?;
            if n == 0 {
                return Err(ReadError::BadRequest);
            }
            remainder.extend_from_slice(&temp[..n]);
        }
        remainder.truncate(len);
        remainder
    } else {
        Vec::new()
    };

    Ok(ParsedRequest {
        method: headers.method,
        path: headers.path,
        body,
    })
}

async fn read_chunked_body<R: AsyncRead + Unpin>(
    reader: &mut R,
    buf: &mut Vec<u8>,
) -> Result<Vec<u8>, ReadError> {
    let mut temp = [0u8; 8192];
    loop {
        match decode_chunked_body(buf) {
            ChunkDecode::Complete(decoded) => return Ok(decoded),
            ChunkDecode::TooLarge => return Err(ReadError::PayloadTooLarge),
            ChunkDecode::Malformed => return Err(ReadError::BadRequest),
            ChunkDecode::Incomplete => {
                if buf.len() > MAX_BODY_BYTES + 64 * 1024 {
                    return Err(ReadError::PayloadTooLarge);
                }
                let n = reader.read(&mut temp).await.map_err(|_| ReadError::Io)?;
                if n == 0 {
                    return Err(ReadError::BadRequest);
                }
                buf.extend_from_slice(&temp[..n]);
            }
        }
    }
}

fn find_header_end(buf: &[u8]) -> Option<usize> {
    buf.windows(4).position(|w| w == b"\r\n\r\n")
}

/// Pure HTTP/1.x request-line + header parser.
fn parse_header_block(raw: &[u8]) -> Result<HeaderBlock, ReadError> {
    let text = std::str::from_utf8(raw).map_err(|_| ReadError::BadRequest)?;
    let mut lines = text.split("\r\n");
    let request_line = lines.next().ok_or(ReadError::BadRequest)?;
    let mut parts = request_line.split_whitespace();
    let method = parts
        .next()
        .ok_or(ReadError::BadRequest)?
        .to_ascii_uppercase();
    let target = parts.next().ok_or(ReadError::BadRequest)?;
    let _version = parts.next().ok_or(ReadError::BadRequest)?;

    let path = target.split('?').next().unwrap_or(target).to_string();

    let mut content_length = None;
    let mut is_chunked = false;
    let mut expect_continue = false;

    for line in lines {
        if line.is_empty() {
            continue;
        }
        let Some((name, value)) = line.split_once(':') else {
            return Err(ReadError::BadRequest);
        };
        let name = name.trim().to_ascii_lowercase();
        let value = value.trim();
        match name.as_str() {
            "content-length" => {
                let parsed = value.parse::<usize>().map_err(|_| ReadError::BadRequest)?;
                content_length = Some(parsed);
            }
            "transfer-encoding" => {
                if value
                    .split(',')
                    .any(|token| token.trim().eq_ignore_ascii_case("chunked"))
                {
                    is_chunked = true;
                }
            }
            "expect" => {
                if value.eq_ignore_ascii_case("100-continue") {
                    expect_continue = true;
                }
            }
            _ => {}
        }
    }

    Ok(HeaderBlock {
        method,
        path,
        content_length,
        is_chunked,
        expect_continue,
    })
}

#[derive(Debug, PartialEq, Eq)]
enum ChunkDecode {
    Complete(Vec<u8>),
    Incomplete,
    TooLarge,
    Malformed,
}

/// Pure incremental HTTP/1.1 chunked transfer decoder.
fn decode_chunked_body(raw: &[u8]) -> ChunkDecode {
    let mut out = Vec::new();
    let mut cursor = 0;

    loop {
        let Some(rel_crlf) = raw[cursor..].windows(2).position(|w| w == b"\r\n") else {
            return ChunkDecode::Incomplete;
        };
        let size_line = match std::str::from_utf8(&raw[cursor..cursor + rel_crlf]) {
            Ok(s) => s,
            Err(_) => return ChunkDecode::Malformed,
        };
        let size_hex = size_line.split(';').next().unwrap_or("").trim();
        let Ok(chunk_len) = usize::from_str_radix(size_hex, 16) else {
            return ChunkDecode::Malformed;
        };
        cursor += rel_crlf + 2;

        if chunk_len == 0 {
            // Require trailing CRLF (or trailers ending in CRLFCRLF).
            if raw[cursor..].starts_with(b"\r\n")
                || raw[cursor..].windows(4).any(|w| w == b"\r\n\r\n")
            {
                return ChunkDecode::Complete(out);
            }
            return ChunkDecode::Incomplete;
        }

        if out.len().saturating_add(chunk_len) > MAX_BODY_BYTES {
            return ChunkDecode::TooLarge;
        }

        let needed = cursor.saturating_add(chunk_len).saturating_add(2);
        if raw.len() < needed {
            return ChunkDecode::Incomplete;
        }

        out.extend_from_slice(&raw[cursor..cursor + chunk_len]);
        cursor += chunk_len;

        if &raw[cursor..cursor + 2] != b"\r\n" {
            return ChunkDecode::Malformed;
        }
        cursor += 2;
    }
}

fn route_request(req: &ParsedRequest, tx: &Sender<EngineEvent>) -> Vec<u8> {
    match (req.method.as_str(), req.path.as_str()) {
        ("GET", "/health") => http_json_response(200, "OK", r#"{"status":"ok"}"#),
        ("POST", CLAUDE_CODE_HOOK_PATH) => {
            match serde_json::from_slice::<ClaudeHookPayload>(&req.body) {
                Ok(payload) => {
                    if let Err(err) = tx.try_send(EngineEvent::ClaudeCodeEvent(payload)) {
                        debug!("Dropping hook event (engine channel busy): {err}");
                    }
                    // Empty JSON object tells Claude Code we have no decision to impose.
                    http_json_response(200, "OK", "{}")
                }
                Err(err) => {
                    debug!("Invalid hook JSON payload: {err}");
                    http_json_response(400, "Bad Request", r#"{"error":"invalid json"}"#)
                }
            }
        }
        (_, CLAUDE_CODE_HOOK_PATH) => http_json_response(
            405,
            "Method Not Allowed",
            r#"{"error":"method not allowed"}"#,
        ),
        _ => http_json_response(404, "Not Found", r#"{"error":"not found"}"#),
    }
}

fn http_json_response(status: u16, reason: &str, body: &str) -> Vec<u8> {
    format!(
        "HTTP/1.1 {status} {reason}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    )
    .into_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::mpsc;

    #[test]
    fn parses_headers_case_insensitively() {
        let raw = b"POST /hooks/claude-code?v=1 HTTP/1.1\r\nHost: 127.0.0.1:47892\r\ncontent-length: 42\r\nExpect: 100-continue";
        let parsed = parse_header_block(raw).unwrap();
        assert_eq!(parsed.method, "POST");
        assert_eq!(parsed.path, "/hooks/claude-code");
        assert_eq!(parsed.content_length, Some(42));
        assert!(!parsed.is_chunked);
        assert!(parsed.expect_continue);
    }

    #[test]
    fn decodes_chunked_transfer_encoding() {
        let raw =
            b"7\r\n{\"hook_\r\n19\r\nevent_name\":\"Stop\",\"cwd\":\r\n6\r\n\"/a\"}\n\r\n0\r\n\r\n";
        assert_eq!(
            decode_chunked_body(raw),
            ChunkDecode::Complete(b"{\"hook_event_name\":\"Stop\",\"cwd\":\"/a\"}\n".to_vec())
        );

        let partial = b"5\r\nhel";
        assert_eq!(decode_chunked_body(partial), ChunkDecode::Incomplete);
    }

    #[tokio::test]
    async fn tcp_server_accepts_content_length_and_chunked_hooks_and_health() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let (tx, mut rx) = mpsc::channel::<EngineEvent>(16);

        let server_task = tokio::spawn(serve_listener(listener, tx));

        // 1. Health check
        let resp = send_raw(addr, b"GET /health HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\n").await;
        assert!(resp.starts_with("HTTP/1.1 200 OK"));
        assert!(resp.ends_with(r#"{"status":"ok"}"#));

        // 2. Content-Length POST hook
        let body = r#"{"session_id":"s1","hook_event_name":"PreToolUse","cwd":"/work/demo","tool_name":"Edit","tool_input":{"file_path":"/work/demo/src/lib.rs"}}"#;
        let req = format!(
            "POST /hooks/claude-code HTTP/1.1\r\nHost: 127.0.0.1\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        );
        let resp = send_raw(addr, req.as_bytes()).await;
        assert!(resp.starts_with("HTTP/1.1 200 OK"));
        assert!(resp.ends_with("{}"));

        match rx.recv().await.unwrap() {
            EngineEvent::ClaudeCodeEvent(p) => {
                assert_eq!(p.session_id, "s1");
                assert_eq!(p.hook_event_name, "PreToolUse");
                assert_eq!(p.tool_name.as_deref(), Some("Edit"));
            }
            other => panic!("Unexpected event: {other:?}"),
        }

        // 3. Chunked POST hook
        let chunk_body = r#"{"session_id":"s1","hook_event_name":"SessionEnd"}"#;
        let chunked_req = format!(
            "POST /hooks/claude-code HTTP/1.1\r\nHost: 127.0.0.1\r\nTransfer-Encoding: chunked\r\n\r\n{:x}\r\n{}\r\n0\r\n\r\n",
            chunk_body.len(),
            chunk_body
        );
        let resp = send_raw(addr, chunked_req.as_bytes()).await;
        assert!(resp.starts_with("HTTP/1.1 200 OK"));

        match rx.recv().await.unwrap() {
            EngineEvent::ClaudeCodeEvent(p) => {
                assert_eq!(p.hook_event_name, "SessionEnd");
            }
            other => panic!("Unexpected event: {other:?}"),
        }

        // 4. Malformed JSON returns 400
        let bad_req =
            b"POST /hooks/claude-code HTTP/1.1\r\nHost: 127.0.0.1\r\nContent-Length: 3\r\n\r\n{{{";
        let resp = send_raw(addr, bad_req).await;
        assert!(resp.starts_with("HTTP/1.1 400 Bad Request"));

        server_task.abort();
    }

    async fn send_raw(addr: std::net::SocketAddr, bytes: &[u8]) -> String {
        let mut stream = TcpStream::connect(addr).await.unwrap();
        stream.write_all(bytes).await.unwrap();
        let mut response = Vec::new();
        stream.read_to_end(&mut response).await.unwrap();
        String::from_utf8(response).unwrap()
    }
}
