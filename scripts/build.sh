#!/bin/bash
set -e

echo "🎄 Building Christmas Lights App..."

# Run trunk build
echo "Running trunk build..."
trunk build

echo "✅ Build complete! Output in ./dist"
