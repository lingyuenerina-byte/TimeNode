**English** | [简体中文](README.md)
# TimeNode (刻度)

TimeNode is a desktop-based timecode and progress bar rendering engine. It generates professional video assets containing smooth progress bars, chapter titles, and scrolling timecodes based on user-defined markers. These assets can be seamlessly overlaid in post-production software like Premiere Pro or DaVinci Resolve.

## Features

* **Smooth Sub-pixel Rendering**: Utilizes WebGL sub-pixel anti-aliasing to calculate progress, eliminating jitter and ensuring ultra-smooth progress bar animation even at low pixel movement speeds.
* **Professional Export Formats**: Supports exporting standard H.264 green screen videos (MP4) and Apple ProRes 4444 videos with a full alpha channel (MOV) for professional workflows.
* **Highly Customizable Design**: Provides an intuitive GUI to tweak track dimensions, colors, borders, typography, and supports mounting custom cursor images (Mascots).
* **Isolated Local File Management**:
  * **Portable Fonts Loading**: Automatically reads and renders font files (TTF/OTF/WOFF) from the `fonts` folder in the root directory without requiring OS-level installation.
  * **Preset System**: All style configurations are saved as JSON files in the local `presets` folder, supporting easy export and import across devices.

## Installation & Setup

This application is a standalone, portable executable. No development environment setup is required.

1. Download the standalone `TimeNode.exe` file.
2. **⚠️ Required Dependency**: The video rendering pipeline relies on FFmpeg. You must download `ffmpeg.exe` and place it in the exact same directory as `TimeNode.exe` (or add it to your system's PATH environment variable).
3. Double-click the executable to launch the app.

## Folder Structure Notes

Upon the first launch, the application will automatically generate the following folders in the same directory as the executable. Ensure the application has write permissions for its folder:
* `/fonts`: Drop your custom font files here to use them directly in the app.
* `/presets`: Stores your saved `.json` style configurations.
* `/output`: The default directory for exported videos.