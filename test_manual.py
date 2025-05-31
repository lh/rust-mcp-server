#!/usr/bin/env python3
"""
Simple test script for the Rust MCP Server
Sends JSON-RPC requests and displays responses
"""

import json
import subprocess
import sys
import time

def send_request(process, request):
    """Send a JSON-RPC request to the server"""
    request_str = json.dumps(request) + '\n'
    print(f"Sending: {request_str.strip()}")
    
    process.stdin.write(request_str.encode())
    process.stdin.flush()
    
    # Read response
    response_line = process.stdout.readline().decode().strip()
    if response_line:
        try:
            response = json.loads(response_line)
            print(f"Response: {json.dumps(response, indent=2)}")
            return response
        except json.JSONDecodeError as e:
            print(f"Failed to parse response: {e}")
            print(f"Raw response: {response_line}")
    else:
        print("No response received")
    return None

def main():
    # Start the server
    server_cmd = ["./target/release/rust-mcp-server", "--project-path", "."]
    
    try:
        process = subprocess.Popen(
            server_cmd,
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            cwd="/Users/rose/Code/brazier/rust-mcp-server"
        )
        
        print("Started Rust MCP Server")
        time.sleep(0.5)  # Give server time to start
        
        # Test 1: Initialize
        print("\n=== Test 1: Initialize ===")
        init_request = {
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {}
        }
        send_request(process, init_request)
        
        # Test 2: List tools
        print("\n=== Test 2: List Tools ===")
        list_request = {
            "jsonrpc": "2.0",
            "id": 2,
            "method": "tools/list",
            "params": {}
        }
        send_request(process, list_request)
        
        # Clean shutdown
        process.terminate()
        process.wait(timeout=5)
        
    except subprocess.TimeoutExpired:
        process.kill()
        print("Server killed due to timeout")
    except Exception as e:
        print(f"Error: {e}")
        if 'process' in locals():
            process.terminate()

if __name__ == "__main__":
    main()