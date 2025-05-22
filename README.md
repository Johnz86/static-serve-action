# Rust Static Server Action

Ultra-minimal memory footprint static file server for GitHub Actions, built with Rust.

## Features

- 🦀 **Rust-powered**: Minimal memory usage (~1-2MB)
- ⚡ **Fast startup**: Ready in seconds
- 🔒 **Secure**: No unnecessary dependencies
- 🌐 **CORS enabled**: Perfect for testing
- 📁 **Directory serving**: Supports any static content

## Usage

```yaml
- name: Start static server
  uses: johnz86/static-serve-action@v1
  with:
    directory: './test'    # Optional, defaults to '.'
    port: '8080'           # Optional, defaults to '8080'
    host: '127.0.0.1'      # Optional, defaults to '127.0.0.1'
```