#!/bin/sh
set -e

SERVE_PATH=${1:-.}
PORT=${2:-8080}
HOST=${3:-127.0.0.1}

echo "🚀 Starting Rust Static Server"
echo "📁 Directory: $SERVE_PATH"
echo "🌐 Host: $HOST"
echo "🔌 Port: $PORT"
echo "📍 URL: http://localhost:$PORT"

# Validate directory exists
if [ ! -d "$SERVE_PATH" ]; then
    echo "❌ Error: Directory '$SERVE_PATH' does not exist"
    exit 1
fi

# Start the server in background and capture PID
static-server -d "$SERVE_PATH" -p "$PORT" --host "$HOST" &
SERVER_PID=$!

# Set up signal handling for graceful shutdown
trap 'echo "🛑 Shutting down server..."; kill $SERVER_PID 2>/dev/null || true; exit 0' TERM INT

# Wait for server to start
sleep 2

# Check if server is actually running
if ! kill -0 $SERVER_PID 2>/dev/null; then
    echo "❌ Failed to start server"
    exit 1
fi

echo "✅ Server started successfully"

# Output for GitHub Actions
if [ -n "$GITHUB_OUTPUT" ]; then
    echo "server-url=http://localhost:$PORT" >> $GITHUB_OUTPUT
    echo "server-pid=$SERVER_PID" >> $GITHUB_OUTPUT
fi

# Keep container alive and wait for server process
wait $SERVER_PID