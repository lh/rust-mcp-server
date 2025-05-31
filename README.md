# Rust MCP Server

A Model Context Protocol (MCP) server that provides essential Rust development tools for AI assistants and IDEs. This server enables real-time syntax checking, linting, formatting, testing, and building of Rust projects through the standardized MCP interface.

## Features

### Available Tools

1. **cargo_check** - Syntax and type validation
   - Validates Rust code syntax and type checking
   - Supports workspace-wide checking
   - Provides detailed error messages and suggestions

2. **cargo_clippy** - Code linting 
   - Catches common mistakes and suggests improvements
   - Supports automatic fixes where possible
   - Configurable lint levels and rules

3. **rustfmt** - Code formatting
   - Formats Rust code according to style guidelines
   - Can check formatting without applying changes
   - Supports individual file or project-wide formatting

4. **cargo_test** - Test execution
   - Runs Rust tests with detailed output
   - Supports specific test patterns and packages
   - Configurable output capture for debugging

5. **cargo_build** - Project building
   - Builds Rust projects in debug or release mode
   - Supports workspace builds
   - Provides build status and error details

## Installation

### Prerequisites

- Rust toolchain (rustc, cargo, clippy, rustfmt)
- Tokio async runtime support

### Build from source

```bash
git clone https://github.com/lh/rust-mcp-server
cd rust-mcp-server
cargo build --release
```

## Claude Code Integration

### Quick Install

```bash
# Clone and build the server
git clone https://github.com/lh/rust-mcp-server
cd rust-mcp-server
cargo build --release

# Add to Claude Code (adjust the project path as needed)
claude mcp add rust-tools -s user -- $(pwd)/target/release/rust-mcp-server --project-path ~/Code

# Verify installation
claude mcp list | grep rust-tools
```

### Alternative Installation Methods

#### Using absolute path:
```bash
claude mcp add rust-tools -s user -- /path/to/rust-mcp-server/target/release/rust-mcp-server --project-path /path/to/your/rust/projects
```

#### Project-specific installation:
```bash
# Navigate to your Rust project
cd /path/to/your/rust/project

# Add the server with current directory as project path
claude mcp add rust-tools -s project -- /path/to/rust-mcp-server/target/release/rust-mcp-server --project-path .
```

### Verify Connection

After installation, restart Claude Code and check the server status:
```
/mcp
```

You should see `rust-tools: connected` in the list.

## Usage

### Command Line

```bash
# Run in current directory
./target/release/rust-mcp-server

# Specify project path
./target/release/rust-mcp-server --project-path /path/to/rust/project
```

### Using with Claude Code

Once installed, you can use natural language to invoke Rust tools:

- "Check this Rust code for errors" → runs `cargo check`
- "Lint my Rust project" → runs `cargo clippy`
- "Format this Rust file" → runs `rustfmt`
- "Run the tests" → runs `cargo test`
- "Build the project in release mode" → runs `cargo build --release`

### MCP Integration

The server communicates via JSON-RPC over stdin/stdout following the MCP protocol:

#### Initialize
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "initialize",
  "params": {}
}
```

#### List Available Tools
```json
{
  "jsonrpc": "2.0", 
  "id": 2,
  "method": "tools/list"
}
```

#### Execute Tool
```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "method": "tools/call",
  "params": {
    "name": "cargo_check",
    "arguments": {
      "workspace": true
    }
  }
}
```

### Tool Parameters

#### cargo_check
```json
{
  "target": "",        // Optional specific target
  "workspace": false   // Check all workspace packages
}
```

#### cargo_clippy
```json
{
  "fix": false,        // Auto-apply fixes
  "workspace": false,  // Run on all packages
  "deny": []          // Lint levels to deny
}
```

#### rustfmt
```json
{
  "file": "",         // Specific file to format
  "check": false      // Check without applying
}
```

#### cargo_test
```json
{
  "test_name": "",    // Specific test pattern
  "package": "",      // Specific package
  "workspace": false, // Test all packages
  "nocapture": false  // Don't capture output
}
```

#### cargo_build
```json
{
  "release": false,   // Build in release mode
  "workspace": false  // Build all packages
}
```

## Output Format

All tools return standardized JSON responses:

```json
{
  "success": true,
  "exit_code": 0,
  "stdout": "compilation output...",
  "stderr": "warnings and errors...",
  "command": "cargo check"
}
```

## Integration Examples

### Claude Code Integration

This server is designed to work seamlessly with AI assistants like Claude Code, providing real-time feedback during code development:

1. **Syntax Validation**: Automatically check code as it's written
2. **Lint Suggestions**: Get improvement suggestions and apply fixes
3. **Code Formatting**: Ensure consistent code style
4. **Test Execution**: Validate changes don't break existing functionality
5. **Build Verification**: Confirm project compiles successfully

### VS Code Extension

Can be integrated with VS Code through MCP to provide enhanced Rust development experience alongside existing language servers.

## Error Handling

The server provides detailed error information for:
- Invalid tool names
- Missing required parameters  
- Compilation errors
- Process execution failures
- File system errors

## Troubleshooting

### Server Not Connecting

If you see connection errors in Claude Code:

1. **Ensure the binary is built:**
   ```bash
   cd /path/to/rust-mcp-server
   cargo build --release
   ls -la target/release/rust-mcp-server  # Should exist and be executable
   ```

2. **Check the -- separator:**
   ```bash
   # ✅ Correct
   claude mcp add rust-tools -- /path/to/rust-mcp-server/target/release/rust-mcp-server
   
   # ❌ Incorrect (missing --)
   claude mcp add rust-tools /path/to/rust-mcp-server/target/release/rust-mcp-server
   ```

3. **Test the server manually:**
   ```bash
   cd /path/to/rust-mcp-server
   python3 test_server.py
   ```

4. **Check logs:**
   Look for error messages when running `claude --mcp-debug`

### Common Issues

- **"cargo: command not found"**: Ensure Rust is installed and in your PATH
- **"failed to find cargo.toml"**: Make sure `--project-path` points to a Rust project
- **Server disconnects immediately**: Check that the binary path is absolute, not relative
- **Server connects but no tools available**: This was a bug in v0.1.0 where the server included `"error": null` in successful responses, violating JSON-RPC 2.0 spec. Fixed in v0.1.1.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Implement your changes
4. Add tests if applicable
5. Submit a pull request

## License

MIT License

## Changelog

### v0.1.1 (fix/json-rpc-error-field)
- Fixed JSON-RPC 2.0 compliance issue where `error: null` was incorrectly included in successful responses
- This fix resolves connection issues with Claude Code where the server would connect but tools wouldn't be available
- Added `#[serde(skip_serializing_if = "Option::is_none")]` to the error field in McpResponse

### v0.1.0
- Initial release
- Core MCP server implementation
- Support for cargo check, clippy, rustfmt, test, and build
- JSON-RPC communication over stdin/stdout
- Configurable project path
- Comprehensive error handling