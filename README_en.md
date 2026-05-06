**English** | [简体中文](README.md)
# TimeNode (刻度)

TimeNode is a desktop-based timecode and progress bar rendering tool. It generates video assets containing smooth progress bars, chapter titles, and scrolling timecodes based on user-defined markers, which can be seamlessly overlaid in post-production software like Premiere Pro or DaVinci Resolve.

*Note: The core architecture design and part of the source code for this project were generated with the assistance of Google Gemini.*

## Features

* **Smooth Sub-pixel Rendering**: Utilizes WebGL sub-pixel anti-aliasing to calculate progress, eliminating jitter and ensuring smooth progress bar animation at low pixel movement speeds.
* **Professional Export Formats**: Supports exporting standard H.264 green screen videos (MP4) and Apple ProRes 4444 videos with a full alpha channel (MOV).
* **Customizable Design**: Provides a GUI to tweak track dimensions, colors, borders, typography, and supports mounting custom cursor images.
* **Isolated Local File Management**:
  * **Local Fonts Loading**: Automatically reads and renders font files (TTF/OTF/WOFF) from the `fonts` folder in the root directory without requiring OS-level installation.
  * **Preset System**: Style configurations are saved as JSON files in the local `presets` folder, supporting easy export and import.

## Installation & Setup

This application is a standalone executable.

1. Download the `TimeNode.exe` file.
2. **Required Dependency**: The video rendering pipeline relies on FFmpeg. You must download `ffmpeg.exe` and place it in the exact same directory as `TimeNode.exe` (or add it to your system's PATH).
3. Double-click to run.

## Folder Structure

Upon the first launch, the application will automatically generate the following folders in the same directory:
* `/fonts`: Drop your custom font files here to use them directly in the app.
* `/presets`: Stores your saved `.json` style configurations.
* `/output`: The default directory for exported videos.