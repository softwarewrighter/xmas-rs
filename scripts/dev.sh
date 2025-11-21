#!/bin/bash
set -e

# Project Configuration
PROJECT_PORT=7030

echo "🎄 Starting Christmas Lights App in Development Mode..."

# Check if trunk serve is already running
if lsof -Pi :${PROJECT_PORT} -sTCP:LISTEN -t >/dev/null 2>&1 ; then
    echo "⚠️  Port ${PROJECT_PORT} is already in use. Stopping existing process..."
    kill $(lsof -t -i:${PROJECT_PORT}) 2>/dev/null || true
    sleep 1
fi

# Start trunk serve (with hot reload)
echo "Starting development server with hot reload on http://localhost:${PROJECT_PORT}"
trunk serve
