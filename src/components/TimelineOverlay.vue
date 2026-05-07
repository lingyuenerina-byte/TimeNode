<template>
  <div class="timeline-wrapper" :style="wrapperStyle">
    <div class="progress-track-group" :style="trackGroupStyle">
      
      <div class="xcprogress" :style="progressTrackStyle">
        <!-- 🚀 核心大改：GPU 独立图层加速的进度填充块 -->
        <div class="xcprogress-bar" :style="progressBarStyle"></div>
        
        <!-- 渲染章节标记节点和文字 -->
        <div v-for="(chapter, index) in chapters" :key="index" class="chapter-marker" :style="getChapterMarkerStyle(chapter)">
          <div v-if="index > 0" class="separator" :style="separatorStyle"></div>
          <div class="title" :style="titleTextStyle">{{ chapter.title }}</div>
        </div>
      </div>

      <!-- 游标定位器 (如果开启了该功能) -->
      <div v-if="config.showMascot && config.mascotImage" class="mascot-container" :style="mascotContainerStyle">
        <img :src="config.mascotImage" :class="'pos-' + (config.mascotPosition || 'center')" :style="mascotImageStyle" />
      </div>

    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, type CSSProperties } from 'vue';

// 接收来自 App.vue 传递下来的属性参数
const props = defineProps(['chapters', 'duration', 'currentTime', 'fps', 'config']);

// 计算当前的播放进度浮点数比例 (0 ~ 1)，用于底层矩阵计算
const currentProgressRatio = computed(() => (props.duration > 0 ? (props.currentTime / props.duration) : 0));

// ====== 外层容器动态样式 (加入 CSSProperties 类型断言解决 TS 报错) ======
const wrapperStyle = computed<CSSProperties>(() => ({ padding: `0 ${props.config.padding}px`, width: '100%', height: '100%', display: 'flex', alignItems: 'center', boxSizing: 'border-box' }));
const trackGroupStyle = computed<CSSProperties>(() => ({ height: `${props.config.trackHeight}px`, position: 'relative', width: '100%' }));
const progressTrackStyle = computed<CSSProperties>(() => ({
  backgroundColor: props.config.trackColor, border: props.config.trackBorderWidth > 0 ? `${props.config.trackBorderWidth}px solid ${props.config.trackBorderColor}` : 'none',
  position: 'absolute', inset: 0, overflow: 'hidden', display: 'flex', boxSizing: 'border-box'
}));

// 🚀 预览不卡顿的核心秘密：利用 CSS3 Transform Scale 触发 GPU 加速渲染
const progressBarStyle = computed<CSSProperties>(() => ({ 
  position: 'absolute', 
  left: 0, 
  top: 0,
  backgroundColor: props.config.progressColor, 
  width: '100%', // 宽度锁死满格
  height: '100%', 
  transformOrigin: 'left center', // 从最左侧原点开始进行横向伸缩
  transform: `scaleX(${currentProgressRatio.value})`, // GPU 硬件加速的浮点级顺滑缩放
  willChange: 'transform',
  transition: 'none !important', // 强行解除阻尼动画
  zIndex: 0
}));

// ====== 文字与修饰物动态样式 ======
const separatorStyle = computed<CSSProperties>(() => ({ backgroundColor: props.config.separatorColor, width: `${props.config.separatorWidth || 2}px`, height: '100%', position: 'absolute', left: 0 }));

const titleTextStyle = computed<CSSProperties>(() => ({ 
  color: props.config.textColor, fontSize: `${props.config.textSize}px`, fontWeight: props.config.textWeight, fontFamily: props.config.fontFamily,
  padding: '0 10px', display: '-webkit-box', WebkitLineClamp: 2, WebkitBoxOrient: 'vertical', overflow: 'hidden', 
  textOverflow: 'ellipsis', whiteSpace: 'normal', wordBreak: 'break-all', lineHeight: '1.2', zIndex: 2, textAlign: 'center'
}));

const mascotContainerStyle = computed<CSSProperties>(() => ({
  position: 'absolute', top: 0, height: '100%', width: '0px', 
  left: (currentProgressRatio.value * 100) + '%', // 将浮点比例转为百分比用于游标追踪定位
  zIndex: 10, pointerEvents: 'none',
  willChange: 'left',
  transition: 'none !important'
}));

const mascotImageStyle = computed<CSSProperties>(() => ({ 
  height: `${props.config.trackHeight * (props.config.mascotScale || 1)}px`, display: 'block' 
}));

// ====== 物理引擎逻辑计算 ======

// 将时码字符串 (HH:MM:SS:FF) 转换成该章节在总进度中的百分比位置
function getPercent(timeStr: string) {
  if(!timeStr) return 0;
  const p = timeStr.split(':').map(Number);
  const secs = (p[0]||0)*3600 + (p[1]||0)*60 + (p[2]||0) + (p[3]||0)/props.fps;
  return (secs / props.duration) * 100;
}

// 动态计算出每个章节标题应该放置的起始点绝对位置
function getChapterMarkerStyle(chapter: any): CSSProperties {
  return { 
    position: 'absolute', top: 0, height: '100%', left: getPercent(chapter.time) + '%', 
    width: getChapterWidth(chapter) + '%', display: 'flex', alignItems: 'center', justifyContent: 'center',
    zIndex: 1 // 确保文字压在进度条上面
  };
}

// 动态计算该章节容器的总宽度：从该章节开始，一直延伸到下一章节的起点（或者视频末尾）
function getChapterWidth(chapter: any) {
  const idx = props.chapters.indexOf(chapter);
  const start = getPercent(chapter.time);
  const end = idx + 1 < props.chapters.length ? getPercent(props.chapters[idx+1].time) : 100;
  return Math.max(0, end - start);
}
</script>

<style scoped>
.mascot-container img { position: absolute; object-fit: contain; }
/* 游标的锚点计算：确保无论图片多大，都能精准对齐到当前的播放时间线上 */
.pos-center { top: 50%; left: 0; transform: translate(-50%, -50%); }
.pos-top { bottom: 100%; left: 0; transform: translateX(-50%); }
.pos-left { top: 50%; right: 0; transform: translateY(-50%); }
</style>