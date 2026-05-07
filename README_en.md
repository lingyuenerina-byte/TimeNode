**English** | [简体中文](README.md)
# TimeNode (刻度)

TimeNode is a desktop-based timecode and progress bar rendering tool. It generates video assets containing smooth progress bars, chapter titles, and scrolling timecodes based on user-defined markers, which can be seamlessly overlaid in post-production software like Premiere Pro or DaVinci Resolve.

>**Platform Support**: This project currently only supports **Windows** OS.

*Note: The core architecture design and part of the source code for this project were generated with the assistance of Google Gemini.*

## Features

* **Smooth Sub-pixel Rendering**: Utilizes WebGL sub-pixel anti-aliasing to calculate progress, eliminating jitter and ensuring smooth progress bar animation at low pixel movement speeds.
* **Built-in Rendering Engine**: The installer comes with FFmpeg integrated, providing an out-of-the-box experience without manual environment configuration.
* **Professional Export Formats**: Supports exporting standard H.264 green screen videos (MP4) and Apple ProRes 4444 videos with a full alpha channel (MOV).
* **Customizable Design**: Provides a GUI to tweak track dimensions, colors, borders, typography, and supports mounting custom cursor images (Mascots).

## Installation & Setup

### Option 1: Direct Download (Recommended)
Please go to the [Releases page](../../releases) to download the latest `setup.exe` installer. This version includes all necessary dependencies and is ready to use after installation.

### Option 2: Build from Source (For Developers)
If you wish to compile or modify the code:
1. Clone the repository and run `npm install`.
2. **Dependency Setup**: The build script automatically bundles resources. You MUST download `ffmpeg.exe` and place it directly into the **`src-tauri/binaries/`** directory before running or building.
3. Start development server: `npm run tauri dev`
4. Build executable: `npm run tauri build`

## Folder Structure
Upon first launch, the application generates the following folders in its directory:
* `/fonts`: Drop your custom font files (TTF/OTF/WOFF) here to use them in the app.
* `/presets`: Stores your saved `.json` style configurations.
* `/output`: The default directory for exported videos.