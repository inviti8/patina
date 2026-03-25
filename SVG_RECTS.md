# SVG Rects Implementation Progress

## Current Status: ✅ Working Nearly Perfect
- SVG loads and displays correctly
- **Perfect horizontal fit** in browser window
- **Perfect mobile scaling** at all resolutions
- **Minor issue**: Landscape mode can cut off bottom on mobile

## Optimal SVG Configuration Found
**Creation**: 377x377 px square in Inkscape
**SVG Modifications**:
1. Remove `width` and `height` attributes from `<svg>` tag
2. Set `viewBox="0 0 100 100"` (unitless coordinate system)
3. Add `preserveAspectRatio="xMidYMid slice"`

## Why 377x377 Works Mathematically
**Key Insight**: 377x377 creates a **square aspect ratio (1:1)** that:
- **Normalizes coordinates** to unitless system
- **Prevents aspect ratio distortion** during scaling
- **Provides clean viewBox mapping** to 0-100 coordinate space
- **Scales proportionally** across all viewport sizes

**The Math**:
- Original: 377x377 = 1:1 ratio
- viewBox: "0 0 100 100" = 1:1 ratio  
- CSS scaling maintains 1:1 relationship
- `preserveAspectRatio="xMidYMid slice"` allows vertical fill

## Current Implementation
- **Method**: `dangerous_inner_html` + CSS styling
- **SVG Source**: `/assets/bg.svg` (modified 377x377 → unitless)
- **CSS Approach**: Modern CSS with `object-fit: fill`
- **Scaling Behavior**: Proportional with perfect horizontal fit

## Key Findings
1. **Square SVG dimensions** (377x377) create optimal scaling base
2. **Unitless viewBox** ("0 0 100 100") enables flexible scaling
3. **preserveAspectRatio="xMidYMid slice"** allows vertical fill
4. **CSS specificity**: `!important` required to override SVG attributes
5. **Viewport units**: `100vw/100vh` ensure full-screen coverage

## Landscape Mode Issue
**Problem**: Bottom cutoff when rotating mobile to landscape
**Cause**: `slice` in preserveAspectRatio crops excess content
**Solutions**:
- Use `preserveAspectRatio="xMidYMid meet"` (no cropping, potential empty space)
- Adjust viewBox height for landscape detection
- Create landscape-specific SVG variant

## Next Steps for Full Control
- [ ] Fix landscape mode cropping issue
- [ ] Parse SVG XML to extract individual elements
- [ ] Manipulate specific rects/paths programmatically  
- [ ] Implement dynamic sizing based on container
- [ ] Preserve Inkscape features (gradients, filters)

## Technical Notes
- **viewBox**: "0 0 100 100" defines normalized coordinate system
- **377x377**: Square base prevents aspect ratio distortion
- **preserveAspectRatio**: "xMidYMid slice" = center + fill + crop excess
- **CSS specificity**: `!important` required to override SVG attributes
- **Viewport units**: `100vw/100vh` ensure full-screen coverage

## Architecture Decision
**Data-driven SVG manipulation** confirmed as right approach:
- Keep SVG as structured data source
- Extract and manipulate elements programmatically
- Maintain original SVG features (gradients, transforms)
- Enable dynamic scaling beyond CSS limitations

---
*Last updated: 2025-03-25*
*Status: Near-perfect scaling achieved, landscape issue remaining*
