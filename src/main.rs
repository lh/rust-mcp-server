use clap::{Arg, Command};
use log::{debug, error, info};
use serde_json::json;
use std::io::{self, BufRead, Write};

mod mcp;
mod rust_tools;

use mcp::{McpRequest, McpResponse};
use rust_tools::RustTools;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    
    let app = Command::new("rust-mcp-server")
        .about("Model Context Protocol server for Rust development tools")
        .arg(
            Arg::new("project-path")
                .long("project-path")
                .short('p')
                .value_name("PATH")
                .help("Path to the Rust project directory")
                .default_value(".")
        );

    let matches = app.get_matches();
    let project_path = matches.get_one::<String>("project-path").unwrap();
    
    info!("Starting Rust MCP Server for project: {}", project_path);
    
    let rust_tools = RustTools::new(project_path.to_string());
    
    // MCP communication loop
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdout_lock = stdout.lock();
    
    for line in stdin.lock().lines() {
        let line = line?;
        debug!("Received: {}", line);
        
        match serde_json::from_str::<McpRequest>(&line) {
            Ok(request) => {
                let response = handle_request(&rust_tools, request).await;
                let response_json = serde_json::to_string(&response)?;
                writeln!(stdout_lock, "{}", response_json)?;
                stdout_lock.flush()?;
            }
            Err(e) => {
                error!("Failed to parse request: {}", e);
                let error_response = McpResponse {
                    jsonrpc: "2.0".to_string(),
                    id: None,
                    result: None,
                    error: Some(json!({
                        "code": -32700,
                        "message": "Parse error",
                        "data": format!("{}", e)
                    })),
                };
                let response_json = serde_json::to_string(&error_response)?;
                writeln!(stdout_lock, "{}", response_json)?;
                stdout_lock.flush()?;
            }
        }
    }
    
    Ok(())
}

async fn handle_request(rust_tools: &RustTools, request: McpRequest) -> McpResponse {
    match request.method.as_str() {
        "initialize" => {
            McpResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: Some(json!({
                    "protocolVersion": "2024-11-05",
                    "capabilities": {
                        "tools": {}
                    },
                    "serverInfo": {
                        "name": "rust-mcp-server",
                        "version": "0.1.1"
                    }
                })),
                error: None,
            }
        }
        "tools/list" => {
            let tools = rust_tools.get_available_tools();
            McpResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: Some(json!({ "tools": tools })),
                error: None,
            }
        }
        "tools/call" => {
            match rust_tools.execute_tool(&request).await {
                Ok(result) => McpResponse {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: Some(result),
                    error: None,
                },
                Err(e) => McpResponse {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: None,
                    error: Some(json!({
                        "code": -32603,
                        "message": "Internal error",
                        "data": format!("{}", e)
                    })),
                }
            }
        }
        _ => McpResponse {
            jsonrpc: "2.0".to_string(),
            id: request.id,
            result: None,
            error: Some(json!({
                "code": -32601,
                "message": "Method not found"
            })),
        }
    }
}