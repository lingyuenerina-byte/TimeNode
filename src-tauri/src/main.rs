// 隐藏 Windows 系统下运行时的控制台黑框
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;
use std::io::Write;
use std::fs;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use tauri::Manager; // ++ 新增：用于动态获取打包后的资源绝对路径

// 全局渲染状态管理器，利用 Mutex 保证多线程情况下的内存安全
pub struct RenderState {
    pub child: Mutex<Option<std::process::Child>>,          // FFmpeg 子进程句柄
    pub stdin: Mutex<Option<std::process::ChildStdin>>,     // 传给 FFmpeg 的图像数据流管道
}

// 字体数据结构体，用于将本地字体文件打包后传给 Vue 前端
#[derive(Serialize, Deserialize)]
struct FontData {
    name: String,
    bytes: Vec<u8>,
}

/**
 * 启动 FFmpeg 渲染进程
 * 核心原理：打开 FFmpeg 并建立 image2pipe 管道，等待前端传入连续的图像帧
 */
#[tauri::command]
async fn start_ffmpeg_render(
    ffmpeg_path: String, // 前端传来的备用路径
    fps: f32,
    output_path: String,
    format: String,
    hardware_accel: String,
    state: tauri::State<'_, RenderState>,
    app: tauri::AppHandle, // ++ 新增：获取当前应用的系统句柄
) -> Result<(), String> {
    
    // 🚀 核心修改：动态定位被打包进安装包里的 FFmpeg
    let mut bundled_ffmpeg = app.path().resource_dir().map_err(|e| e.to_string())?;
    bundled_ffmpeg.push("binaries");
    bundled_ffmpeg.push("ffmpeg.exe");

    // 智能寻路：如果找到了安装包自带的就用自带的，如果没找到就回退使用前端传来的路径
    let target_exe = if bundled_ffmpeg.exists() {
        bundled_ffmpeg.to_string_lossy().to_string()
    } else {
        ffmpeg_path
    };

    // 判断图像格式：绿幕使用极速的 mjpeg 传输，透明底必须使用 png 保留 Alpha 通道
    let input_codec = if format == "mp4_green" { "mjpeg" } else { "png" };

    let mut args = vec![
        "-y".to_string(),                               // 强制覆盖已存在的同名文件
        "-f".to_string(), "image2pipe".to_string(),     // 声明输入源为持续的图像管道
        "-vcodec".to_string(), input_codec.to_string(), // 声明管道输入图像格式
        "-r".to_string(), fps.to_string(),              // 设定目标输出帧率
        "-thread_queue_size".to_string(), "1024".to_string(), // 扩大缓冲队列，防止高帧率下管道阻塞
        "-i".to_string(), "-".to_string(),              // 从标准输入 (stdin) 读取流数据
    ];

    // 根据不同格式采用不同的输出编码器
    if format == "mov_prores" {
        // MOV 格式：采用影视工业标准 ProRes 4444，支持完美透明通道
        args.extend(vec![
            "-c:v".to_string(), "prores_ks".to_string(),
            "-profile:v".to_string(), "4444".to_string(),
            "-pix_fmt".to_string(), "yuva444p10le".to_string(),
        ]);
    } else {
        // MP4 绿幕格式：分为 NVENC 硬件加速和 X264 CPU 软压
        if hardware_accel == "nvidia" {
            args.extend(vec![
                "-c:v".to_string(), "h264_nvenc".to_string(),
                "-preset".to_string(), "p4".to_string(),
                "-tune".to_string(), "hq".to_string(),
                "-pix_fmt".to_string(), "yuv420p".to_string(),
            ]);
        } else {
            // CPU x264：兼容性最强，保底策略
            args.extend(vec![
                "-c:v".to_string(), "libx264".to_string(),
                "-preset".to_string(), "veryfast".to_string(),
                "-pix_fmt".to_string(), "yuv420p".to_string(),
            ]);
        }
    }

    args.push(output_path); // 追加最终输出的物理路径

    // 唤起命令行子进程 (此时 target_exe 已经是绝对路径了)
    let mut child = std::process::Command::new(target_exe)
        .args(args)
        .stdin(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| format!("无法启动渲染引擎: {}", e))?;

    // 捕获管道句柄并存入全局状态，供后续帧写入
    let stdin = child.stdin.take().ok_or("Failed to open stdin")?;
    *state.child.lock().unwrap() = Some(child);
    *state.stdin.lock().unwrap() = Some(stdin);

    Ok(())
}

/**
 * 核心渲染管道：将前端 WebGL 画出的一帧图像，强制灌入 FFmpeg
 */
#[tauri::command]
async fn send_frame(frame_data: Vec<u8>, state: tauri::State<'_, RenderState>) -> Result<(), String> {
    let mut stdin_lock = state.stdin.lock().unwrap();
    if let Some(ref mut stdin) = *stdin_lock {
        stdin.write_all(&frame_data).map_err(|e| e.to_string())?;
        stdin.flush().map_err(|e| e.to_string())?;
    }
    Ok(())
}

/**
 * 结束渲染：关闭管道，让 FFmpeg 完成封包输出
 */
#[tauri::command]
async fn finish_render(state: tauri::State<'_, RenderState>) -> Result<(), String> {
    let mut stdin_lock = state.stdin.lock().unwrap();
    let mut child_lock = state.child.lock().unwrap();
    *stdin_lock = None; // 切断写入流，触发 EOF
    if let Some(mut child) = child_lock.take() {
        let _ = child.wait(); // 等待压制完成并退出
    }
    Ok(())
}

/**
 * [本地基建] 启动时自动在 exe 目录创建三大核心文件夹：字体、预设、输出
 */
#[tauri::command]
fn init_app_dirs() -> Result<HashMap<String, String>, String> {
    let mut dir = std::env::current_exe().map_err(|e| e.to_string())?;
    dir.pop(); // 回退到 exe 所在根目录
    
    let mut fonts_dir = dir.clone(); fonts_dir.push("fonts");
    let mut presets_dir = dir.clone(); presets_dir.push("presets");
    let mut output_dir = dir.clone(); output_dir.push("output");
    
    // 如果没有文件夹就静默创建
    let _ = fs::create_dir_all(&fonts_dir);
    let _ = fs::create_dir_all(&presets_dir);
    let _ = fs::create_dir_all(&output_dir);
    
    // 传回给 Vue 前端作为绝对路径参考
    let mut map = HashMap::new();
    map.insert("fonts".to_string(), fonts_dir.to_string_lossy().to_string());
    map.insert("presets".to_string(), presets_dir.to_string_lossy().to_string());
    map.insert("output".to_string(), output_dir.to_string_lossy().to_string());
    
    Ok(map)
}

/**
 * 读取本地预设文件夹下的所有 .json 配置文件
 */
#[tauri::command]
fn read_presets(dir: String) -> Result<Vec<String>, String> {
    let mut presets = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            if entry.path().extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(content) = fs::read_to_string(entry.path()) {
                    presets.push(content);
                }
            }
        }
    }
    Ok(presets)
}

// 基础文件操作 API 暴露给前端
#[tauri::command]
fn write_file_rust(path: String, content: String) -> Result<(), String> {
    fs::write(path, content).map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_preset_file(path: String) -> Result<(), String> {
    fs::remove_file(path).map_err(|e| e.to_string())
}

#[tauri::command]
fn read_file_rust(path: String) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| e.to_string())
}

/**
 * 加载并读取 fonts 文件夹下的所有字体 (ttf, otf, woff)
 * 读取后将其转化为字节流传给前端进行动态 CSS 挂载
 */
#[tauri::command]
fn load_local_fonts() -> Result<Vec<FontData>, String> {
    let mut dir = std::env::current_exe().map_err(|e| e.to_string())?;
    dir.pop();
    dir.push("fonts");
    if !dir.exists() {
        let _ = fs::create_dir_all(&dir);
        return Ok(vec![]);
    }
    
    let mut fonts = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                let ext_lower = ext.to_lowercase();
                if ext_lower == "ttf" || ext_lower == "otf" || ext_lower == "woff" || ext_lower == "woff2" {
                    if let Ok(bytes) = fs::read(&path) {
                        let name = path.file_stem().unwrap().to_string_lossy().to_string();
                        fonts.push(FontData { name, bytes }); // 记录字体名称与二进制数据
                    }
                }
            }
        }
    }
    Ok(fonts)
}

/**
 * 唤起系统原生资源管理器，直达字体文件夹
 */
#[tauri::command]
fn open_fonts_dir() -> Result<(), String> {
    let mut dir = std::env::current_exe().map_err(|e| e.to_string())?;
    dir.pop();
    dir.push("fonts");
    if !dir.exists() { let _ = fs::create_dir_all(&dir); }
    // 跨平台资源管理器适配
    #[cfg(target_os = "windows")]
    std::process::Command::new("explorer").arg(&dir).spawn().map_err(|e| e.to_string())?;
    #[cfg(target_os = "macos")]
    std::process::Command::new("open").arg(&dir).spawn().map_err(|e| e.to_string())?;
    Ok(())
}

/**
 * 唤起系统默认浏览器打开 GitHub 仓库 (极致稳定版)
 */
#[tauri::command]
fn open_github() -> Result<(), String> {
    let url = "https://github.com/lingyuenerina-byte/TimeNode";
    
    // Windows 下使用 rundll32 避免 cmd /c start 的解析 bug
    #[cfg(target_os = "windows")]
    std::process::Command::new("rundll32")
        .args(["url.dll,FileProtocolHandler", url])
        .spawn()
        .map_err(|e| e.to_string())?;
    
    #[cfg(target_os = "macos")]
    std::process::Command::new("open").arg(url).spawn().map_err(|e| e.to_string())?;
    
    Ok(())
}

// 主程序入口与命令总线注册
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init()) // 注册原生文件选择框插件
        .manage(RenderState { child: Mutex::new(None), stdin: Mutex::new(None) })
        .invoke_handler(tauri::generate_handler![
            start_ffmpeg_render, send_frame, finish_render,
            write_file_rust, load_local_fonts, open_fonts_dir,
            init_app_dirs, read_presets, delete_preset_file, read_file_rust,
            open_github
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}