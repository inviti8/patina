#!/usr/bin/env python3
"""
SVG Post-Processing Script for Patina UI Framework
Automatically removes width/height and adds preserveAspectRatio to SVG files
"""

import os
import re
from pathlib import Path

def process_svg_file(svg_path: Path):
    """Process a single SVG file to remove width/height and add preserveAspectRatio"""
    
    if not svg_path.exists():
        print(f"❌ SVG file not found: {svg_path}")
        return False
    
    # Read original SVG content
    with open(svg_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    original_content = content
    
    # Remove width attribute
    content = re.sub(r'\s*width="[^"]*"', '', content)
    
    # Remove height attribute  
    content = re.sub(r'\s*height="[^"]*"', '', content)
    
    # Add preserveAspectRatio if not present
    if 'preserveAspectRatio=' not in content:
        # Find the <svg> tag and add preserveAspectRatio after viewBox
        svg_tag_pattern = r'(<svg[^>]*viewBox="[^"]*"[^>]*>)'
        def add_preserve_aspect(match):
            svg_tag = match.group(1)
            if 'preserveAspectRatio=' not in svg_tag:
                # Insert preserveAspectRatio after viewBox
                svg_tag = svg_tag.replace('viewBox="', 'preserveAspectRatio="xMidYMid slice" viewBox="')
            return svg_tag
        
        content = re.sub(svg_tag_pattern, add_preserve_aspect, content)
    
    # Only write if content changed
    if content != original_content:
        with open(svg_path, 'w', encoding='utf-8') as f:
            f.write(content)
        print(f"✅ Processed: {svg_path}")
        return True
    else:
        print(f"⏭️  No changes needed: {svg_path}")
        return False

def main():
    """Process all SVG files in the assets directory"""
    
    # Find all SVG files in assets directory
    assets_dir = Path("assets")
    svg_files = list(assets_dir.glob("**/*.svg"))
    
    if not svg_files:
        print("❌ No SVG files found in assets directory")
        return
    
    print(f"🔧 Processing {len(svg_files)} SVG files...")
    
    processed_count = 0
    for svg_file in svg_files:
        if process_svg_file(svg_file):
            processed_count += 1
    
    print(f"\n✨ Complete! Processed {processed_count}/{len(svg_files)} SVG files")

if __name__ == "__main__":
    main()
