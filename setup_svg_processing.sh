#!/bin/bash

# SVG Post-Processing Hook for Patina UI Framework
# This script automatically processes SVG files when they change

echo "🔧 Setting up SVG post-processing..."

# Make the Python script executable
chmod +x process_svg.py

# Create a git hook to automatically process SVGs on commit
if [ ! -d ".git/hooks" ]; then
    mkdir -p .git/hooks
fi

# Pre-commit hook to process SVGs
cat > .git/hooks/pre-commit << 'EOF'
#!/bin/bash
echo "🔧 Processing SVG files before commit..."
python3 process_svg.py
EOF

chmod +x .git/hooks/pre-commit

# Create a watch script for development
cat > watch_svg.sh << 'EOF'
#!/bin/bash

echo "👀 Watching SVG files for changes..."
echo "Press Ctrl+C to stop"

# Install inotify-tools if not present
if ! command -v inotifywait &> /dev/null; then
    echo "Installing inotify-tools..."
    sudo apt-get update && sudo apt-get install -y inotify-tools
fi

while true; do
    inotifywait -e modify,create,moved_to --recursive assets/ --include='*.svg' |
    while read path action file; do
        echo "📝 SVG changed: $path$file ($action)"
        python3 process_svg.py
        echo "✅ SVG processed successfully"
    done
done
EOF

chmod +x watch_svg.sh

echo "✅ SVG post-processing setup complete!"
echo ""
echo "Usage:"
echo "  Run once:     python3 process_svg.py"
echo "  Watch for changes: ./watch_svg.sh"
echo "  Auto on commit: Git pre-commit hook installed"
