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
git clone <repository>
cd rust-mcp-server
cargo build --release
```

## Usage

### Command Line

```bash
# Run in current directory
./target/release/rust-mcp-server

# Specify project path
./target/release/rust-mcp-server --project-path /path/to/rust/project
```

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

## Contributing

1. Fork the repository
2. Create a feature branch
3. Implement your changes
4. Add tests if applicable
5. Submit a pull request

## License

[License information]

## Changelog

### v0.1.0
- Initial release
- Core MCP server implementation
- Support for cargo check, clippy, rustfmt, test, and build
- JSON-RPC communication over stdin/stdout
- Configurable project path
- Comprehensive error handling