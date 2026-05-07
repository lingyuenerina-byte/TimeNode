/**
 * 将计算机内部的纯秒数 (例如: 125.433 秒) 
 * 格式化输出为影视工业标准时码 (HH:MM:SS:FF)
 */
export function formatTimecode(totalSeconds: number, currentFps: number): string {
  const h = Math.floor(totalSeconds / 3600);
  const m = Math.floor((totalSeconds % 3600) / 60);
  const s = Math.floor(totalSeconds % 60);
  
  // 计算当前帧数：提取秒数的小数部分，乘以视频帧率，并向下取整
  const f = Math.floor((totalSeconds - Math.floor(totalSeconds)) * currentFps);
  
  // 补零占位符，保证两位对齐输出
  const pad = (n: number) => n.toString().padStart(2, '0');
  return `${pad(h)}:${pad(m)}:${pad(s)}:${pad(f)}`;
}

/**
 * 将用户手动输入的 PR 标准时码 (HH:MM:SS:FF) 
 * 解析回计算机底层物理引擎所需的纯秒数 (Float 浮点型)
 */
export function timecodeToSeconds(timeStr: string, currentFps: number): number {
  if (!timeStr) return 0;
  const parts = timeStr.toString().split(':').map(Number);
  const [h = 0, m = 0, s = 0, f = 0] = parts;
  // 帧数转化为秒数的小数部分
  return h * 3600 + m * 60 + s + (f / currentFps);
}

/**
 * 为页面上的章节列表动态生成极简带圈序号 (①, ②, ③...)
 * 如果章节数量超过 20，Unicode 字符集不够用了，则降级显示为 (21)
 */
export function getCircledNumber(index: number): string {
  // Unicode 0x2460 开始是带圈的数字 1
  if (index < 20) return String.fromCharCode(0x2460 + index); 
  return `(${index + 1})`;
}