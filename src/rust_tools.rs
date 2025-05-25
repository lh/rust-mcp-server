use crate::mcp::{McpRequest, Tool, ToolCall};
use anyhow::{anyhow, Result};
use log::debug;
use serde_json::{json, Value};
use std::path::PathBuf;
use std::process::Stdio;
use tokio::process::Command;

pub struct RustTools {
    project_path: PathBuf,
}

impl RustTools {
    pub fn new(project_path: String) -> Self {
        Self {
            project_path: PathBuf::from(project_path),
        }
    }

    pub fn get_available_tools(&self) -> Vec<Tool> {
        vec![
            Tool {
                name: "cargo_check".to_string(),
                description: "Run `cargo check` to validate Rust code syntax and type checking".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "target": {
                            "type": "string",
                            "description": "Optional target to check (e.g., specific file or package)",
                            "default": ""
                        },
                        "workspace": {
                            "type": "boolean",
                            "description": "Check all packages in the workspace",
                            "default": false
                        }
                    }
                })
            },
            Tool {
                name: "cargo_clippy".to_string(),
                description: "Run `cargo clippy` to lint Rust code and catch common mistakes".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "fix": {
                            "type": "boolean",
                            "description": "Automatically apply suggested fixes where possible",
                            "default": false
                        },
                        "workspace": {
                            "type": "boolean",
                            "description": "Run clippy on all packages in the workspace",
                            "default": false
                        },
                        "deny": {
                            "type": "array",
                            "items": {"type": "string"},
                            "description": "Lint levels to deny (e.g., ['warnings', 'clippy::pedantic'])",
                            "default": []
                        }
                    }
                })
            },
            Tool {
                name: "rustfmt".to_string(),
                description: "Format Rust code using rustfmt".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "file": {
                            "type": "string",
                            "description": "Specific file to format (if empty, formats all files)",
                            "default": ""
                        },
                        "check": {
                            "type": "boolean",
                            "description": "Check if files are formatted without applying changes",
                            "default": false
                        }
                    }
                })
            },
            Tool {
                name: "cargo_test".to_string(),
                description: "Run Rust tests using `cargo test`".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "test_name": {
                            "type": "string",
                            "description": "Specific test name or pattern to run",
                            "default": ""
                        },
                        "package": {
                            "type": "string",
                            "description": "Specific package to test",
                            "default": ""
                        },
                        "workspace": {
                            "type": "boolean",
                            "description": "Run tests for all packages in the workspace",
                            "default": false
                        },
                        "nocapture": {
                            "type": "boolean",
                            "description": "Don't capture test output (useful for debugging)",
                            "default": false
                        }
                    }
                })
            },
            Tool {
                name: "cargo_build".to_string(),
                description: "Build the Rust project using `cargo build`".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "release": {
                            "type": "boolean",
                            "description": "Build in release mode",
                            "default": false
                        },
                        "workspace": {
                            "type": "boolean",
                            "description": "Build all packages in the workspace",
                            "default": false
                        }
                    }
                })
            }
        ]
    }

    pub async fn execute_tool(&self, request: &McpRequest) -> Result<Value> {
        let params = request.params.as_ref().ok_or_else(|| anyhow!("Missing params"))?;
        let tool_call: ToolCall = serde_json::from_value(params.clone())?;
        
        debug!("Executing tool: {}", tool_call.name);

        match tool_call.name.as_str() {
            "cargo_check" => self.run_cargo_check(tool_call.arguments).await,
            "cargo_clippy" => self.run_cargo_clippy(tool_call.arguments).await,
            "rustfmt" => self.run_rustfmt(tool_call.arguments).await,
            "cargo_test" => self.run_cargo_test(tool_call.arguments).await,
            "cargo_build" => self.run_cargo_build(tool_call.arguments).await,
            _ => Err(anyhow!("Unknown tool: {}", tool_call.name)),
        }
    }

    async fn run_cargo_check(&self, args: Option<Value>) -> Result<Value> {
        let mut cmd = Command::new("cargo");
        cmd.arg("check")
           .current_dir(&self.project_path)
           .stdout(Stdio::piped())
           .stderr(Stdio::piped());

        if let Some(args) = args {
            if let Some(workspace) = args.get("workspace").and_then(|v| v.as_bool()) {
                if workspace {
                    cmd.arg("--workspace");
                }
            }
            if let Some(target) = args.get("target").and_then(|v| v.as_str()) {
                if !target.is_empty() {
                    cmd.arg("--").arg(target);
                }
            }
        }

        let output = cmd.output().await?;
        
        Ok(json!({
            "success": output.status.success(),
            "exit_code": output.status.code().unwrap_or(-1),
            "stdout": String::from_utf8_lossy(&output.stdout),
            "stderr": String::from_utf8_lossy(&output.stderr),
            "command": "cargo check"
        }))
    }

    async fn run_cargo_clippy(&self, args: Option<Value>) -> Result<Value> {
        let mut cmd = Command::new("cargo");
        cmd.arg("clippy")
           .current_dir(&self.project_path)
           .stdout(Stdio::piped())
           .stderr(Stdio::piped());

        if let Some(args) = args {
            if let Some(fix) = args.get("fix").and_then(|v| v.as_bool()) {
                if fix {
                    cmd.arg("--fix");
                }
            }
            if let Some(workspace) = args.get("workspace").and_then(|v| v.as_bool()) {
                if workspace {
                    cmd.arg("--workspace");
                }
            }
            if let Some(deny_array) = args.get("deny").and_then(|v| v.as_array()) {
                for deny_item in deny_array {
                    if let Some(lint) = deny_item.as_str() {
                        cmd.arg("--").arg("-D").arg(lint);
                    }
                }
            }
        }

        let output = cmd.output().await?;
        
        Ok(json!({
            "success": output.status.success(),
            "exit_code": output.status.code().unwrap_or(-1),
            "stdout": String::from_utf8_lossy(&output.stdout),
            "stderr": String::from_utf8_lossy(&output.stderr),
            "command": "cargo clippy"
        }))
    }

    async fn run_rustfmt(&self, args: Option<Value>) -> Result<Value> {
        let mut cmd = Command::new("cargo");
        cmd.arg("fmt")
           .current_dir(&self.project_path)
           .stdout(Stdio::piped())
           .stderr(Stdio::piped());

        if let Some(args) = args {
            if let Some(check) = args.get("check").and_then(|v| v.as_bool()) {
                if check {
                    cmd.arg("--check");
                }
            }
            if let Some(file) = args.get("file").and_then(|v| v.as_str()) {
                if !file.is_empty() {
                    cmd.arg("--").arg(file);
                }
            }
        }

        let output = cmd.output().await?;
        
        Ok(json!({
            "success": output.status.success(),
            "exit_code": output.status.code().unwrap_or(-1),
            "stdout": String::from_utf8_lossy(&output.stdout),
            "stderr": String::from_utf8_lossy(&output.stderr),
            "command": "cargo fmt"
        }))
    }

    async fn run_cargo_test(&self, args: Option<Value>) -> Result<Value> {
        let mut cmd = Command::new("cargo");
        cmd.arg("test")
           .current_dir(&self.project_path)
           .stdout(Stdio::piped())
           .stderr(Stdio::piped());

        if let Some(args) = args {
            if let Some(test_name) = args.get("test_name").and_then(|v| v.as_str()) {
                if !test_name.is_empty() {
                    cmd.arg(test_name);
                }
            }
            if let Some(package) = args.get("package").and_then(|v| v.as_str()) {
                if !package.is_empty() {
                    cmd.arg("--package").arg(package);
                }
            }
            if let Some(workspace) = args.get("workspace").and_then(|v| v.as_bool()) {
                if workspace {
                    cmd.arg("--workspace");
                }
            }
            if let Some(nocapture) = args.get("nocapture").and_then(|v| v.as_bool()) {
                if nocapture {
                    cmd.arg("--").arg("--nocapture");
                }
            }
        }

        let output = cmd.output().await?;
        
        Ok(json!({
            "success": output.status.success(),
            "exit_code": output.status.code().unwrap_or(-1),
            "stdout": String::from_utf8_lossy(&output.stdout),
            "stderr": String::from_utf8_lossy(&output.stderr),
            "command": "cargo test"
        }))
    }

    async fn run_cargo_build(&self, args: Option<Value>) -> Result<Value> {
        let mut cmd = Command::new("cargo");
        cmd.arg("build")
           .current_dir(&self.project_path)
           .stdout(Stdio::piped())
           .stderr(Stdio::piped());

        if let Some(args) = args {
            if let Some(release) = args.get("release").and_then(|v| v.as_bool()) {
                if release {
                    cmd.arg("--release");
                }
            }
            if let Some(workspace) = args.get("workspace").and_then(|v| v.as_bool()) {
                if workspace {
                    cmd.arg("--workspace");
                }
            }
        }

        let output = cmd.output().await?;
        
        Ok(json!({
            "success": output.status.success(),
            "exit_code": output.status.code().unwrap_or(-1),
            "stdout": String::from_utf8_lossy(&output.stdout),
            "stderr": String::from_utf8_lossy(&output.stderr),
            "command": "cargo build"
        }))
    }
}