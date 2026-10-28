#!/usr/bin/env bash
set -e

echo ""
echo "======================================================="
echo " 🦀 Starting Git-Backed Markdown Blog Development Env  "
echo "======================================================="
echo ""

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

cleanup() {
    echo ""
    echo "🛑 Shutting down backend..."
    if [ -n "$BACKEND_PID" ]; then
        kill "$BACKEND_PID" 2>/dev/null || true
    fi
    pkill -f "git-blog-api" 2>/dev/null || true
    echo "✅ All processes stopped."
}

trap cleanup EXIT INT TERM

# Ensure frontend dependencies are installed
if [ ! -d "$ROOT_DIR/frontend/node_modules" ]; then
    echo "📦 Installing frontend dependencies..."
    (cd "$ROOT_DIR/frontend" && pnpm install)
fi

# Start backend
echo "🚀 Starting Rust/Axum backend on http://127.0.0.1:3000..."
(cd "$ROOT_DIR/backend" && cargo run) &
BACKEND_PID=$!

# Start frontend
echo "⚡ Starting React/Vite frontend on http://localhost:5173..."
echo "💡 Press Ctrl+C at any time to stop both servers."
echo ""

(cd "$ROOT_DIR/frontend" && pnpm dev)

