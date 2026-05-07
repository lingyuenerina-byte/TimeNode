export class RenderEngine {
  private canvas: HTMLCanvasElement;
  private ctx: CanvasRenderingContext2D;
  private glCanvas: HTMLCanvasElement | null = null;
  private gl: WebGLRenderingContext | null = null;
  private glProgram: WebGLProgram | null = null;

  constructor(width: number, height: number) {
    this.canvas = document.createElement('canvas');
    this.canvas.width = width;
    this.canvas.height = height;
    // willReadFrequently 提升从显存读取像素的速度
    this.ctx = this.canvas.getContext('2d', { alpha: true, willReadFrequently: true })!;
  }

  public initWebGL() {
    this.glCanvas = document.createElement('canvas');
    this.glCanvas.width = this.canvas.width;
    this.glCanvas.height = this.canvas.height;
    const gl = this.glCanvas.getContext('webgl', { alpha: true, premultipliedAlpha: false });
    if (!gl) return;
    const vs = gl.createShader(gl.VERTEX_SHADER)!;
    gl.shaderSource(vs, `attribute vec2 p; void main() { gl_Position = vec4(p, 0.0, 1.0); }`);
    gl.compileShader(vs);
    const fs = gl.createShader(gl.FRAGMENT_SHADER)!;
    
    // 🚀 核心突破：引入 smoothstep 实现亚像素抗锯齿渲染，彻底解决缓慢移动时的 0.1 像素卡顿抖动
    gl.shaderSource(fs, `
      precision mediump float; 
      uniform vec2 u_res; 
      uniform float u_prog; 
      uniform vec4 u_bg; 
      uniform vec4 u_fg; 
      void main() { 
        vec2 st = gl_FragCoord.xy / u_res; 
        float pixelWidth = 0.75 / u_res.x; // 获取当前 1 个像素的物理宽度范围
        // 核心：在进度边缘的 1 像素范围内进行平滑透明度混合 (Anti-aliasing)
        float alpha = 1.0 - smoothstep(u_prog - pixelWidth, u_prog + pixelWidth, st.x);
        gl_FragColor = mix(u_bg, u_fg, alpha);
      }
    `);
    
    gl.compileShader(fs);
    const prog = gl.createProgram()!;
    gl.attachShader(prog, vs); gl.attachShader(prog, fs); gl.linkProgram(prog);
    gl.useProgram(prog);
    this.gl = gl; this.glProgram = prog;
    const buf = gl.createBuffer(); gl.bindBuffer(gl.ARRAY_BUFFER, buf);
    gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([-1,-1, 1,-1, -1,1, 1,1]), gl.STATIC_DRAW);
    const pLoc = gl.getAttribLocation(prog, 'p'); gl.enableVertexAttribArray(pLoc);
    gl.vertexAttribPointer(pLoc, 2, gl.FLOAT, false, 0, 0);
  }

  private hex2rgba(hex: string) {
    const num = parseInt(hex.substring(1), 16);
    return [(num >> 16 & 255)/255, (num >> 8 & 255)/255, (num & 255)/255, 1.0];
  }

  public async renderFrame(params: { progress: number, config: any, chapterData: any[], mascotImg: HTMLImageElement | null, format: string }): Promise<ArrayBuffer> {
    const { width, height } = this.canvas;
    const { progress, config, chapterData, mascotImg, format } = params;
    const pad = config.padding || 0;
    const drawW = width - (pad * 2);

    // 填充底色 (绿幕或透明)
    if (format === 'mp4_green') {
        this.ctx.fillStyle = '#00FF00';
        this.ctx.fillRect(0, 0, width, height);
    } else {
        this.ctx.clearRect(0, 0, width, height);
    }

    const trackH = config.trackHeight;
    const trackY = (height - trackH) / 2;

    // 1. 绘制轨道 (带亚像素抗锯齿的 WebGL 渲染)
    if (this.gl && this.glProgram) {
      this.gl.uniform2f(this.gl.getUniformLocation(this.glProgram, 'u_res'), drawW, trackH);
      this.gl.uniform1f(this.gl.getUniformLocation(this.glProgram, 'u_prog'), progress);
      this.gl.uniform4fv(this.gl.getUniformLocation(this.glProgram, 'u_bg'), this.hex2rgba(config.trackColor));
      this.gl.uniform4fv(this.gl.getUniformLocation(this.glProgram, 'u_fg'), this.hex2rgba(config.progressColor));
      this.gl.drawArrays(this.gl.TRIANGLE_STRIP, 0, 4);
      this.ctx.drawImage(this.glCanvas!, pad, trackY, drawW, trackH);
    }

    // 1.5 绘制轨道外框 (补全边缘描边)
    if (config.trackBorderWidth > 0) {
      this.ctx.strokeStyle = config.trackBorderColor;
      this.ctx.lineWidth = config.trackBorderWidth;
      this.ctx.strokeRect(pad, trackY, drawW, trackH);
    }

    // 2. 绘制文字与分割线 (全程保留浮点数，拒绝 Math.floor 造成卡顿)
    this.ctx.font = `${config.textWeight} ${config.textSize}px ${config.fontFamily}`;
    this.ctx.textAlign = 'center'; this.ctx.textBaseline = 'middle';
    
    chapterData.forEach((ch, idx) => {
      const x = pad + (ch.startX / width) * drawW;
      const w = (ch.width / width) * drawW;
      
      // 绘制分割线
      if (idx > 0) { 
        this.ctx.fillStyle = config.separatorColor; 
        this.ctx.fillRect(x, trackY, config.separatorWidth, trackH); 
      }
      
      // 绘制标题
      this.ctx.fillStyle = config.textColor;
      const maxW = w - 20;
      let lines = [];
      let cur = '';
      for (let char of ch.title) {
        if (this.ctx.measureText(cur + char).width < maxW) cur += char;
        else { lines.push(cur); cur = char; }
      }
      lines.push(cur);
      
      const centerY = trackY + trackH / 2;
      if (lines.length === 1) this.ctx.fillText(lines[0], x + w/2, centerY);
      else {
        this.ctx.fillText(lines[0], x + w/2, centerY - config.textSize * 0.5);
        this.ctx.fillText(lines[1].length > 10 ? lines[1].slice(0, 8) + '...' : lines[1], x + w/2, centerY + config.textSize * 0.7);
      }
    });

    // 🚀 3. 补全：绘制游标图 (Mascot)
    if (mascotImg) {
      const mascotH = trackH * (config.mascotScale || 1); // 游标高度
      const mascotW = mascotImg.width * (mascotH / mascotImg.height); // 等比例缩放宽度
      const currentX = pad + drawW * progress; // 当前进度的 X 坐标 (浮点数绝对精准)
      
      let drawX = currentX;
      let drawY = trackY;
      
      // 按照用户设置的对齐方式渲染游标
      if (config.mascotPosition === 'center') {
          drawX -= mascotW / 2;
          drawY -= (mascotH - trackH) / 2;
      } else if (config.mascotPosition === 'top') {
          drawX -= mascotW / 2;
          drawY -= mascotH; // 完全贴在轨道上方
      } else if (config.mascotPosition === 'left') {
          drawX -= mascotW; // 锚点定在右侧，向左延伸
          drawY -= (mascotH - trackH) / 2;
      }
      
      this.ctx.drawImage(mascotImg, drawX, drawY, mascotW, mascotH);
    }

    // 4. 极速导出：JPEG 压缩打破传输瓶颈
    const quality = format === 'mp4_green' ? 0.95 : undefined;
    const type = format === 'mp4_green' ? 'image/jpeg' : 'image/png';
    const blob = await new Promise<Blob>((res) => this.canvas.toBlob(b => res(b!), type, quality));
    return await blob.arrayBuffer();
  }
}