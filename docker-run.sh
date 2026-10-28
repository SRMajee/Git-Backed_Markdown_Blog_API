#!/usr/bin/env bash
set -e

echo ""
echo "======================================================="
echo " 🐳 Launching Git-Backed Markdown Blog in Docker      "
echo "======================================================="
echo ""

if ! command -v docker >/dev/null 2>&1; then
    echo "❌ Docker CLI not found. Please install Docker."
    exit 1
fi

echo "🔍 Checking Docker daemon..."
if ! docker info >/dev/null 2>&1; then
    echo ""
    echo "❌ Docker daemon is not running!"
    echo "👉 Please start the Docker daemon / Docker Desktop and re-run this script."
    echo ""
    exit 1
fi

echo "🚀 Building and starting containers with Docker Compose..."
echo "   - Frontend: http://localhost:8080"
echo "   - Backend API: http://localhost:3000/api/posts"
echo "💡 Press Ctrl+C to stop the containers."
echo ""

docker compose up --build

