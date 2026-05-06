<template>
  <div class="tc-container" :class="{ 'dark-theme': isDark }">
    
    <!-- 🚀 顶栏：完全扁平透明无边框设计 -->
    <div class="tc-top-header">
      <div class="tc-logo">
        <span class="tc-title">{{ locale === 'zh' ? '刻度' : 'TimeNode' }}</span>
        <span class="tc-subtitle">{{ t('subtitle') }}</span>
        <span class="tc-author">{{ t('author') }}: LingyueNerina</span>
      </div>
      
      <!-- 右侧控制区 (主题、语言、GitHub) -->
      <div class="tc-top-actions">
        <button class="tc-flat-btn" @click="toggleThemeMode" :title="t('themeTip')">
          <svg v-if="themeMode === 'auto'" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="2" y="3" width="20" height="14" rx="2" ry="2"/><line x1="8" y1="21" x2="16" y2="21"/><line x1="12" y1="17" x2="12" y2="21"/></svg>
          <svg v-if="themeMode === 'light'" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="5"/><line x1="12" y1="1" x2="12" y2="3"/><line x1="12" y1="21" x2="12" y2="23"/><line x1="4.22" y1="4.22" x2="5.64" y2="5.64"/><line x1="18.36" y1="18.36" x2="19.78" y2="19.78"/><line x1="1" y1="12" x2="3" y2="12"/><line x1="21" y1="12" x2="23" y2="12"/><line x1="4.22" y1="19.78" x2="5.64" y2="18.36"/><line x1="18.36" y1="5.64" x2="19.78" y2="4.22"/></svg>
          <svg v-if="themeMode === 'dark'" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/></svg>
          <span>{{ themeMode === 'auto' ? t('auto') : themeMode === 'light' ? t('light') : t('dark') }}</span>
        </button>
        <button class="tc-flat-btn" @click="toggleLanguage" title="Switch Language">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><path d="M2 12h20M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"/></svg>
          {{ locale === 'zh' ? 'EN' : '中' }}
        </button>
        <!-- 唤起系统浏览器打开 GitHub 仓库 -->
        <button class="tc-flat-btn tc-github" @click="openGithub">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor"><path d="M12 2C6.477 2 2 6.477 2 12c0 4.42 2.865 8.166 6.839 9.489.5.092.682-.217.682-.482 0-.237-.008-.866-.013-1.7-2.782.603-3.369-1.34-3.369-1.34-.454-1.156-1.11-1.464-1.11-1.464-.908-.62.069-.608.069-.608 1.003.07 1.531 1.03 1.531 1.03.892 1.529 2.341 1.087 2.91.831.092-.646.35-1.086.636-1.336-2.22-.253-4.555-1.11-4.555-4.943 0-1.091.39-1.984 1.029-2.683-.103-.253-.446-1.27.098-2.647 0 0 .84-.269 2.75 1.025A9.578 9.578 0 0 1 12 6.836c.85.004 1.705.114 2.504.336 1.909-1.294 2.747-1.025 2.747-1.025.546 1.377.203 2.394.1 2.647.64.699 1.028 1.592 1.028 2.683 0 3.842-2.339 4.687-4.566 4.935.359.309.678.919.678 1.852 0 1.336-.012 2.415-.012 2.743 0 .267.18.578.688.48C19.138 20.161 22 16.416 22 12c0-5.523-4.477-10-10-10z"/></svg>
          GitHub
        </button>
      </div>
    </div>

    <!-- 📺 模拟预览区 -->
    <div class="tc-preview-section">
      <!-- 独立离屏 Canvas 挂载点，用于渲染进度条组件 -->
      <div class="tc-canvas-wrapper">
        <TimelineOverlay class="tc-preview-canvas" :chapters="chapters" :duration="duration" :currentTime="currentTime" :fps="fps" :config="currentThemeConfig" />
      </div>
      
      <!-- 播放控制台 -->
      <div class="tc-playback-bar">
        <button class="tc-btn-play" @click="togglePlay" :class="{ 'is-playing': isPlaying }">
          <svg v-if="!isPlaying" width="14" height="14" viewBox="0 0 24 24" fill="currentColor"><path d="M8 5v14l11-7z"/></svg>
          <svg v-else width="14" height="14" viewBox="0 0 24 24" fill="currentColor"><path d="M6 19h4V5H6v14zm8-14v14h4V5h-4z"/></svg>
          {{ isPlaying ? t('pause') : t('play') }}
        </button>
        <!-- 进度条滑块：step="any" 解除原生精度截断限制 -->
        <div class="tc-slider-wrap">
          <input type="range" v-model.number="currentTime" :max="duration" step="any" class="tc-range-slider" @mousedown="onDragStart" @mouseup="onDragEnd" />
        </div>
        <!-- 纯文本时码解析器 -->
        <div class="tc-timecode-box">
          <input 
            v-model.lazy="currentTimeInput" 
            @focus="isPlaying = false"
            @change="onCurrentTimeChange" 
            class="tc-time-input-inline" 
            :title="t('tcHint')"
          />
          <span class="tc-time-sep">/</span>
          <span class="tc-time-total">{{ durationInput }}</span>
        </div>
      </div>
    </div>

    <!-- 🛠️ 工程核心参数栏 -->
    <div class="tc-toolbar">
      <div class="tc-toolbar-item">
        <span class="tc-label">{{ t('duration') }}</span>
        <input v-model.lazy="durationInput" @change="onDurationChange" class="tc-input text-center" style="width: 105px;" />
      </div>
      
      <!-- 🚀 纯手绘：视频宽度下拉菜单 -->
      <div class="tc-toolbar-item">
        <span class="tc-label">{{ t('resolution') }}</span>
        <div class="tc-custom-select-wrapper" style="width: 175px;">
          <div class="tc-custom-select-display" :class="{'is-open': isResOpen}" @click="isResOpen = !isResOpen">
            <span>{{ currentResLabel }}</span>
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#6b778c" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" :style="{ transform: isResOpen ? 'rotate(180deg)' : 'rotate(0)' }" style="transition: 0.2s;"><polyline points="6 9 12 15 18 9"/></svg>
          </div>
          <div class="tc-custom-select-backdrop" v-if="isResOpen" @click="isResOpen = false"></div>
          <div class="tc-custom-select-menu" v-if="isResOpen">
            <div class="tc-custom-select-optgroup">{{ t('screen169') }}</div>
            <div class="tc-custom-select-option" :class="{'is-active': videoResolution==='1920'}" @click="selectRes('1920')">{{ t('fhd') }}</div>
            <div class="tc-custom-select-option" :class="{'is-active': videoResolution==='2560'}" @click="selectRes('2560')">{{ t('qhd') }}</div>
            <div class="tc-custom-select-option" :class="{'is-active': videoResolution==='3840'}" @click="selectRes('3840')">{{ t('uhd') }}</div>
            <div class="tc-custom-select-optgroup">{{ t('screen219') }}</div>
            <div class="tc-custom-select-option" :class="{'is-active': videoResolution==='2560_uw'}" @click="selectRes('2560_uw')">{{ t('uw1080') }}</div>
            <div class="tc-custom-select-option" :class="{'is-active': videoResolution==='3440_uw'}" @click="selectRes('3440_uw')">{{ t('uw1440') }}</div>
          </div>
        </div>
      </div>
      
      <!-- 🚀 纯手绘：FPS 帧率下拉菜单 -->
      <div class="tc-toolbar-item">
        <span class="tc-label">{{ t('fps') }}</span>
        <div class="tc-custom-select-wrapper" style="width: 75px;">
          <div class="tc-custom-select-display" :class="{'is-open': isFpsOpen}" @click="isFpsOpen = !isFpsOpen">
            <span>{{ fps }}</span>
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#6b778c" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" :style="{ transform: isFpsOpen ? 'rotate(180deg)' : 'rotate(0)' }" style="transition: 0.2s;"><polyline points="6 9 12 15 18 9"/></svg>
          </div>
          <div class="tc-custom-select-backdrop" v-if="isFpsOpen" @click="isFpsOpen = false"></div>
          <div class="tc-custom-select-menu" v-if="isFpsOpen">
             <div v-for="f in [24, 25, 30, 50, 59.94, 60]" :key="f" class="tc-custom-select-option" :class="{'is-active': fps === f}" @click="selectFps(f)">{{ f }}</div>
          </div>
        </div>
      </div>
      
      <!-- 输出目录选择器 -->
      <div class="tc-toolbar-item flex-1" style="min-width: 180px;">
        <span class="tc-label">{{ t('save') }}</span>
        <div class="tc-path-group">
          <input v-model="savePath" class="tc-input flex-1" readonly @click.stop="selectFolder" />
          <button class="tc-flat-btn tc-icon-only" @click.stop.prevent="selectFolder" title="选择目录">
             <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/></svg>
          </button>
        </div>
      </div>

      <!-- 🚀 纯手绘：输出格式下拉菜单 -->
      <div class="tc-toolbar-item">
        <span class="tc-label">{{ t('format') }}</span>
        <div class="tc-custom-select-wrapper" style="width: 155px;">
          <div class="tc-custom-select-display" :class="{'is-open': isFormatOpen}" @click="isFormatOpen = !isFormatOpen">
            <span style="color: #00A896; font-weight: bold;">{{ currentFormatLabel }}</span>
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#6b778c" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" :style="{ transform: isFormatOpen ? 'rotate(180deg)' : 'rotate(0)' }" style="transition: 0.2s;"><polyline points="6 9 12 15 18 9"/></svg>
          </div>
          <div class="tc-custom-select-backdrop" v-if="isFormatOpen" @click="isFormatOpen = false"></div>
          <div class="tc-custom-select-menu" v-if="isFormatOpen">
             <div class="tc-custom-select-option" style="color: #00A896; font-weight: bold;" :class="{'is-active': videoFormat === 'mp4_green'}" @click="selectFormat('mp4_green')">{{ t('green') }}</div>
             <div class="tc-custom-select-option" style="color: #00A896; font-weight: bold;" :class="{'is-active': videoFormat === 'mov_prores'}" @click="selectFormat('mov_prores')">{{ t('alpha') }}</div>
          </div>
        </div>
      </div>
    </div>

    <!-- 左右双栏工作区：数据注入层 与 样式渲染层 -->
    <div class="tc-workspace">
      <!-- 📌 左侧面板：章节节点数据源 -->
      <div class="tc-card panel-left flex-col">
        <div class="tc-card-header">
          <div class="tc-card-title">
            <div style="display:flex; align-items:center; gap:8px;">
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#F56C6C" stroke-width="2"><path d="M12 20h9M16.5 3.5a2.121 2.121 0 0 1 3 3L7 19l-4 1 1-4L16.5 3.5z"/></svg>
              {{ t('chapters') }} <span class="tc-sub">TIMECODES</span>
            </div>
            <!-- 时码冲突警告标识 -->
            <div class="tc-chapter-hint-flat" :class="{'is-error': hasChapterError}">
              <span class="dot"></span>
              {{ hasChapterError ? t('hintErr') : t('hintOk') }}
            </div>
          </div>
        </div>
        
        <div class="tc-card-body flex-col flex-1 overflow-hidden" style="padding: 20px;">
          <div class="tc-table-header">
            <div class="col-idx">{{ t('idx') }}</div><div class="col-time">{{ t('tc') }}</div><div class="col-title">{{ t('title') }}</div><div class="col-act"></div>
          </div>
          <div class="tc-chapter-list">
            <div v-for="(ch, i) in chapters" :key="ch.id" class="tc-chapter-row">
              <div class="col-idx">{{ getCircledNumber(i) }}</div> 
              <!-- 错误时间码将触发红框警告 -->
              <input v-model="ch.time" class="tc-input font-mono text-center col-time" :class="{'is-error': !isChapterValid(ch.time, i)}" />
              <input v-model="ch.title" class="tc-input col-title" />
              <button class="tc-flat-btn tc-del-btn col-act" @click="chapters.splice(i, 1)">
                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="3 6 5 6 21 6"/><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/></svg>
              </button>
            </div>
          </div>
          <button class="tc-btn-dashed w-full mt-3" @click="addChapter">{{ t('addChapter') }}</button>
        </div>
        
        <!-- 一键渲染起爆点 -->
        <div class="tc-card-footer">
          <button class="tc-btn-mega" @click="handleGenerate">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M4.5 16.5c-1.5 1.26-2 5-2 5s3.74-.5 5-2c.71-.84.7-2.13-.09-2.91a2.18 2.18 0 0 0-2.91-.09z"/><path d="m12 15-3-3a22 22 0 0 1 2-3.95A12.88 12.88 0 0 1 22 2c0 2.72-.78 7.5-6 11a22.35 22.35 0 0 1-4 2z"/><path d="M9 12H4s.55-3.03 2-4c1.62-1.08 5 0 5 0"/><path d="M12 15v5s3.03-.55 4-2c1.08-1.62 0-5 0-5"/></svg>
            {{ t('startRender') }}
          </button>
        </div>
      </div>

      <!-- 🎨 右侧面板：外观精调与渲染底层参数 -->
      <div class="tc-card panel-right flex-col">
        <div class="tc-card-header">
          <div class="tc-card-title">
            <div style="display:flex; align-items:center; gap:8px;">
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#E6A23C" stroke-width="2"><circle cx="12" cy="12" r="10"/><path d="M12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 0-20"/><path d="M2 12h20"/></svg>
              {{ t('style') }} <span class="tc-sub">DESIGN & STYLE</span>
            </div>
          </div>
        </div>
        
        <div class="tc-card-body overflow-y" style="padding: 25px 35px;">
          
          <!-- 本地预设文件系统 (向左聚拢设计) -->
          <div class="tc-preset-bar">
            <span class="tc-label">{{ t('preset') }}</span>
            <div class="tc-custom-select-wrapper" style="width: 220px;">
              <div class="tc-custom-select-display" :class="{'is-open': isPresetOpen}" @click="isPresetOpen = !isPresetOpen">
                <span style="overflow:hidden; text-overflow:ellipsis; white-space:nowrap;">{{ currentThemeName }}</span>
                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#6b778c" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" :style="{ transform: isPresetOpen ? 'rotate(180deg)' : 'rotate(0)' }" style="transition: 0.2s;"><polyline points="6 9 12 15 18 9"/></svg>
              </div>
              <div class="tc-custom-select-backdrop" v-if="isPresetOpen" @click="isPresetOpen = false"></div>
              <div class="tc-custom-select-menu" v-if="isPresetOpen">
                <template v-if="builtInThemes.length">
                  <div class="tc-custom-select-optgroup">{{ t('sysPreset') }}</div>
                  <div class="tc-custom-select-option" v-for="t in builtInThemes" :key="t.id" :class="{'is-active': currentThemeId === t.id}" @click="selectTheme(t.id)">{{ t.name }}</div>
                </template>
                <template v-if="customThemes.length">
                  <div class="tc-custom-select-optgroup">{{ t('myPreset') }}</div>
                  <div class="tc-custom-select-option" v-for="t in customThemes" :key="t.id" :class="{'is-active': currentThemeId === t.id}" @click="selectTheme(t.id)">{{ t.name }}</div>
                </template>
              </div>
            </div>

            <!-- 预设操作栏 (含安全重命名) -->
            <div class="tc-theme-actions">
              <span class="tc-action-link" @click="addTheme">{{ t('add') }}</span>
              <span class="tc-action-link" @click="saveTheme">{{ t('savePreset') }}</span>
              <span class="tc-action-link" @click="renameTheme">{{ t('rename') }}</span>
              <span class="tc-action-link" @click="exportTheme">{{ t('export') }}</span>
              <span class="tc-action-link" @click="importThemeTrigger">{{ t('import') }}</span>
              <span class="tc-action-link danger" @click="deleteTheme">{{ t('del') }}</span>
            </div>
          </div>

          <!-- 游标（Mascot）挂载系统 -->
          <div class="tc-preset-bar" style="justify-content: flex-end; margin-top: -5px;">
            <span class="tc-label">{{ t('mascot') }}</span>
            <label class="tc-switch"><input type="checkbox" v-model="currentThemeConfig.showMascot"><span class="slider"></span></label>
            
            <!-- 🚀 纯手绘：游标位置选择器 (安全防闪退) -->
            <div class="tc-custom-select-wrapper" style="width: 75px; margin: 0 5px;" :class="{ 'is-disabled': !currentThemeConfig.showMascot }">
              <div class="tc-custom-select-display" :class="{'is-open': isMascotPosOpen}" @click="currentThemeConfig.showMascot && (isMascotPosOpen = !isMascotPosOpen)">
                <span>{{ currentMascotPosLabel }}</span>
                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#6b778c" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" :style="{ transform: isMascotPosOpen ? 'rotate(180deg)' : 'rotate(0)' }" style="transition: 0.2s;"><polyline points="6 9 12 15 18 9"/></svg>
              </div>
              <div class="tc-custom-select-backdrop" v-if="isMascotPosOpen" @click="isMascotPosOpen = false"></div>
              <div class="tc-custom-select-menu" v-if="isMascotPosOpen">
                 <div class="tc-custom-select-option" :class="{'is-active': currentThemeConfig.mascotPosition === 'center'}" @click="selectMascotPos('center')">{{ t('center') }}</div>
                 <div class="tc-custom-select-option" :class="{'is-active': currentThemeConfig.mascotPosition === 'top'}" @click="selectMascotPos('top')">{{ t('top') }}</div>
                 <div class="tc-custom-select-option" :class="{'is-active': currentThemeConfig.mascotPosition === 'left'}" @click="selectMascotPos('left')">{{ t('left') }}</div>
              </div>
            </div>

            <!-- 游标图片上传按钮 -->
            <div class="tc-flat-btn tc-icon-only" @click="triggerMascotUpload" :class="{ 'is-disabled': !currentThemeConfig.showMascot }" style="overflow:hidden;" title="点击上传图片">
              <img v-if="currentThemeConfig.mascotImage" :src="currentThemeConfig.mascotImage" style="width:100%;height:100%;object-fit:contain;" />
              <svg v-else width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="3" y="3" width="18" height="18" rx="2" ry="2"/><circle cx="8.5" cy="8.5" r="1.5"/><polyline points="21 15 16 10 5 21"/></svg>
              <input type="file" ref="mascotFile" style="display:none" accept="image/*" @change="onMascotUpload" />
            </div>
          </div>

          <div class="tc-divider" style="margin: 15px 0;"></div>

          <!-- 🎨 渲染引擎参数绑定区 (强制偶数步长 step="2"，避免非整数切割产生黑边) -->
          <div class="tc-settings-grid">
            <div class="tc-cell"><div class="tc-cell-head"><span>{{ t('progColor') }}</span><div class="tc-color-box"><input type="color" v-model="currentThemeConfig.progressColor" /><span>{{currentThemeConfig.progressColor.toUpperCase()}}</span></div></div></div>
            <div class="tc-cell"><div class="tc-cell-head"><span>{{ t('trackBg') }}</span><div class="tc-color-box"><input type="color" v-model="currentThemeConfig.trackColor" /><span>{{currentThemeConfig.trackColor.toUpperCase()}}</span></div></div></div>
            
            <div class="tc-cell"><div class="tc-cell-head"><span>{{ t('padding') }}</span><span class="tc-val">{{currentThemeConfig.padding}}px</span></div><input type="range" v-model.number="currentThemeConfig.padding" min="0" max="100" step="2" class="tc-slider-native" /></div>
            <div class="tc-cell"><div class="tc-cell-head"><span>{{ t('trackHeight') }}</span><span class="tc-val">{{currentThemeConfig.trackHeight}}px</span></div><input type="range" v-model.number="currentThemeConfig.trackHeight" min="20" max="150" step="2" class="tc-slider-native" /></div>
            
            <div class="tc-cell"><div class="tc-cell-head"><span>{{ t('borderWidth') }}</span><span class="tc-val">{{currentThemeConfig.trackBorderWidth}}px</span></div><input type="range" v-model.number="currentThemeConfig.trackBorderWidth" min="0" max="20" step="2" class="tc-slider-native" /></div>
            <div class="tc-cell"><div class="tc-cell-head"><span>{{ t('borderColor') }}</span><div class="tc-color-box"><input type="color" v-model="currentThemeConfig.trackBorderColor" /></div></div></div>
            
            <!-- 极客暗夜预设的分割线颜色对比度已修复 -->
            <div class="tc-cell"><div class="tc-cell-head"><span>{{ t('sepWidth') }}</span><span class="tc-val">{{currentThemeConfig.separatorWidth}}px</span></div><input type="range" v-model.number="currentThemeConfig.separatorWidth" min="2" max="10" step="2" class="tc-slider-native" /></div>
            <div class="tc-cell"><div class="tc-cell-head"><span>{{ t('sepColor') }}</span><div class="tc-color-box"><input type="color" v-model="currentThemeConfig.separatorColor" /><span>{{currentThemeConfig.separatorColor.toUpperCase()}}</span></div></div></div>
            
            <div class="tc-cell"><div class="tc-cell-head"><span>{{ t('mascotScale') }}</span><span class="tc-val">{{currentThemeConfig.mascotScale}}x</span></div><input type="range" v-model.number="currentThemeConfig.mascotScale" min="0.5" max="3" step="0.1" class="tc-slider-native" /></div>
            <div class="tc-cell"></div> <!-- 占位空白单元格，维持网格的严格对称美学 -->

            <div class="tc-cell" style="grid-column: 1 / -1"><div class="tc-divider" style="margin: 5px 0;"></div></div>
            
            <div class="tc-cell"><div class="tc-cell-head"><span>{{ t('textSize') }}</span><span class="tc-val">{{currentThemeConfig.textSize}}px</span></div><input type="range" v-model.number="currentThemeConfig.textSize" min="12" max="60" step="2" class="tc-slider-native" /></div>
            <div class="tc-cell"><div class="tc-cell-head"><span>{{ t('textColor') }}</span><div class="tc-color-box"><input type="color" v-model="currentThemeConfig.textColor" /></div></div></div>
            
            <!-- 文字粗细定制下拉菜单 (设置 is-up 向上弹出，防止底部截断) -->
            <div class="tc-cell">
              <div class="tc-cell-head" style="margin-bottom: 8px;"><span>{{ t('textWeight') }}</span></div>
              <div class="tc-custom-select-wrapper w-full">
                <div class="tc-custom-select-display" :class="{'is-open': isWeightOpen}" @click="isWeightOpen = !isWeightOpen">
                  <span>{{ currentWeightLabel }}</span>
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#6b778c" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" :style="{ transform: isWeightOpen ? 'rotate(180deg)' : 'rotate(0)' }" style="transition: 0.2s;"><polyline points="6 9 12 15 18 9"/></svg>
                </div>
                <div class="tc-custom-select-backdrop" v-if="isWeightOpen" @click="isWeightOpen = false"></div>
                <div class="tc-custom-select-menu is-up" v-if="isWeightOpen">
                  <div class="tc-custom-select-option" :class="{'is-active': currentThemeConfig.textWeight === 'normal'}" @click="selectWeight('normal')">{{ t('normal') }}</div>
                  <div class="tc-custom-select-option" :class="{'is-active': currentThemeConfig.textWeight === 'bold'}" @click="selectWeight('bold')">{{ t('bold') }}</div>
                  <div class="tc-custom-select-option" :class="{'is-active': currentThemeConfig.textWeight === '900'}" @click="selectWeight('900')">{{ t('heavy') }}</div>
                </div>
              </div>
            </div>

            <!-- 本地字体挂载系统下拉菜单 (向上弹出) -->
            <div class="tc-cell">
              <div class="tc-cell-head" style="margin-bottom: 8px;"><span>{{ t('font') }}</span></div>
              <div class="tc-path-group">
                <div class="tc-custom-select-wrapper flex-1">
                  <div class="tc-custom-select-display" :class="{'is-open': isFontOpen}" @click="isFontOpen = !isFontOpen">
                    <span style="overflow:hidden; text-overflow:ellipsis; white-space:nowrap;">{{ currentFontLabel }}</span>
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="#6b778c" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" :style="{ transform: isFontOpen ? 'rotate(180deg)' : 'rotate(0)' }" style="transition: 0.2s;"><polyline points="6 9 12 15 18 9"/></svg>
                  </div>
                  <div class="tc-custom-select-backdrop" v-if="isFontOpen" @click="isFontOpen = false"></div>
                  <div class="tc-custom-select-menu is-up" v-if="isFontOpen">
                    <div class="tc-custom-select-option" :class="{'is-active': currentThemeConfig.fontFamily === 'system-ui'}" @click="selectFont('system-ui')">{{ t('sysFont') }}</div>
                    <div class="tc-custom-select-optgroup" v-if="customFonts.length">{{ t('localFonts') }}</div>
                    <div class="tc-custom-select-option" v-for="f in customFonts" :key="f.value" :class="{'is-active': currentThemeConfig.fontFamily === f.value}" @click="selectFont(f.value)">{{ f.label }}</div>
                  </div>
                </div>
                <!-- 唤起系统资源管理器，指向本地的 fonts 目录 -->
                <button class="tc-flat-btn tc-icon-only" @click="openFontsDir" title="打开本地字体文件夹">
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/></svg>
                </button>
                <!-- 重新读取 fonts 目录内的所有字体，并注入到 DOM -->
                <button class="tc-flat-btn tc-icon-only" @click="loadLocalFontsDir" title="刷新本地字体">
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="23 4 23 10 17 10"/><polyline points="1 20 1 14 7 14"/><path d="M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15"/></svg>
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 🚀 压制进程监控器 (全局遮罩弹窗) -->
    <div class="tc-overlay" v-if="isRendering">
      <div class="tc-modal render-modal">
        <h3 class="modal-title">{{ t('rendering') }}</h3>
        <div class="tc-progress-wrap"><div class="tc-progress-bar" :style="{ width: renderProgress + '%' }"></div></div>
        <div class="modal-stats">
          <span>{{ t('progress') }}: {{ renderProgress.toFixed(1) }}%</span>
          <span style="color: #00A896">{{ t('remain') }}: {{ renderEta }}</span>
        </div>

        <div class="tc-log-toggle" @click="showRenderLogs = !showRenderLogs">
          {{ showRenderLogs ? t('hideLog') : t('showLog') }}
        </div>
        
        <!-- 压制参数底层日志与状态反馈 -->
        <div class="tc-log-panel" v-if="showRenderLogs">
          <div class="log-info-grid">
             <div class="log-info-item"><span>{{ t('resolution') }}:</span> {{ videoResolution === '2560_uw' ? '2560x1080' : videoResolution === '3440_uw' ? '3440x1440' : videoResolution + 'x' + Math.floor(parseInt(videoResolution)*9/16) }}</div>
             <div class="log-info-item"><span>{{ t('fps') }}:</span> {{ fps }} fps</div>
             <div class="log-info-item"><span>{{ t('encoder') }}:</span> {{ videoFormat === 'mov_prores' ? 'ProRes 4444' : 'x264 CPU' }}</div>
             <div class="log-info-item"><span>{{ t('taskInfo') }}:</span> {{ chapters.length }} {{ t('chapters') }}</div>
          </div>
          <div class="log-line log-hl">{{ t('initEngine') }}</div>
          <div class="log-line">{{ currentFrameLog }}</div>
        </div>
      </div>
    </div>

    <!-- 🚀 统一风格的系统安全确认弹窗 -->
    <div class="tc-overlay" style="z-index: 10000" v-if="customDialog.show">
      <div class="tc-modal dialog-modal">
        <div class="dialog-header">{{ customDialog.title }}</div>
        <div class="dialog-body">{{ customDialog.message }}</div>
        <input v-if="customDialog.type === 'prompt'" v-model="customDialog.input" class="tc-input w-full mt-3" @keyup.enter="resolveDialog(true)" />
        <div class="dialog-footer">
          <button v-if="customDialog.type !== 'alert'" class="dialog-btn-cancel" @click="resolveDialog(false)">{{ t('cancel') }}</button>
          <button class="dialog-btn-confirm" @click="resolveDialog(true)">{{ t('ok') }}</button>
        </div>
      </div>
    </div>

  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { open, save } from '@tauri-apps/plugin-dialog';
import TimelineOverlay from './components/TimelineOverlay.vue';
import { formatTimecode, timecodeToSeconds, getCircledNumber } from './utils/timecode';
import { RenderEngine } from './engine/RenderEngine';

// ====== 多语言环境字典配置 ======
const locale = ref('zh');
const toggleLanguage = () => { 
  locale.value = locale.value === 'zh' ? 'en' : 'zh'; 
  localStorage.setItem('timelineCraftLocale', locale.value);
};

const dict: Record<string, Record<string, string>> = {
  zh: {
    subtitle: '时码渲染引擎', author: '作者', themeTip: '切换外观模式', auto: '系统', light: '浅色', dark: '深色',
    play: '播放', pause: '暂停', duration: '总时长', resolution: '视频宽度', fps: 'FPS', save: '保存至', format: '格式',
    chapters: '章节节点', hintOk: '时码需递增且在时长内', hintErr: '时码冲突或超出时长', tcHint: '手动输入时码并回车以跳转，输入时自动暂停',
    idx: '序', tc: '时码', title: '标题', act: '操作', addChapter: '+ 添加新章节', newChapter: '新章节', startRender: '启动引擎渲染导出', saveHint: '点击选择输出目录...',
    style: '样式精调', preset: '预设', sysPreset: '系统内置', myPreset: '我的配置', add: '新增', savePreset: '保存', del: '删除', export: '导出', import: '导入', mascot: '游标图',
    rename: '重命名', renamePrompt: '请输入新的预设名称：', cannotRenameBuiltin: '系统内置预设无法重命名',
    center: '居中', top: '上方', left: '左侧', delPresetMsg: '确认删除当前选中的预设吗？', presetNamePrompt: '请输入新预设的名称：', customThemeDefault: '新自定义主题', blank: '空白预设', importSuffix: '导入',
    saveConfirmMsg: '确认覆盖保存当前预设吗？', stop: '拦截提示', noSavePath: '请先在上方设置输出保存位置！', timecodeErr: '章节时码存在格式错误、溢出或冲突，请修正红色高亮的时间点！',
    success: '成功', renderSuccess: '视频已成功导出！可以前往对应目录查看。', error: '系统异常', 
    showLog: '显示详细参数与日志 ▼', hideLog: '隐藏详细参数与日志 ▲', taskInfo: '任务数', encoder: '编码引擎', initEngine: '[系统] WebGL 渲染核心已就绪', ffmpegFrame: '>> [FFmpeg] 正在压制并写入第 {frame} 帧数据...',
    progColor: '进度条颜色', trackBg: '轨道底色', padding: '内边距', trackHeight: '轨道高度', 
    borderWidth: '外框粗细', borderColor: '外框颜色', sepWidth: '分割线粗细', sepColor: '分割线颜色', mascotScale: '游标缩放',
    textSize: '文字大小', textColor: '文字颜色', textWeight: '文字粗细', font: '字体集',
    normal: '常规', bold: '加粗 (Bold)', heavy: '特粗', sysFont: '系统原生', localFonts: '本地字体库',
    rendering: '正在极速压制中...', progress: '整体进度', remain: '预计剩余', cancel: '取消', ok: '确定',
    fhd: '1920x1080 (FHD)', qhd: '2560x1440 (2K)', uhd: '3840x2160 (4K)',
    uw1080: '2560x1080 (带鱼屏)', uw1440: '3440x1440 (带鱼屏)',
    green: 'MP4 (自加绿幕)', alpha: 'MOV (透明背景)', screen169: '常规屏幕 (16:9)', screen219: '超宽屏幕 (21:9)'
  },
  en: {
    subtitle: 'Timecode Engine', author: 'Author', themeTip: 'Toggle Theme', auto: 'Auto', light: 'Light', dark: 'Dark',
    play: 'Play', pause: 'Pause', duration: 'Duration', resolution: 'Resolution', fps: 'FPS', save: 'Save To', format: 'Format',
    chapters: 'Timecodes', hintOk: 'Valid & Incremental', hintErr: 'Conflict or Overflow', tcHint: 'Input timecode & Enter to jump. Pauses automatically.',
    idx: 'No.', tc: 'Timecode', title: 'Title', act: 'Act', addChapter: '+ Add Chapter', newChapter: 'New Chapter', startRender: 'Start Render Export', saveHint: 'Select output dir...',
    style: 'Design & Style', preset: 'Preset', sysPreset: 'Built-in', myPreset: 'My Config', add: 'New', savePreset: 'Save', del: 'Delete', export: 'Export', import: 'Import', mascot: 'Mascot',
    rename: 'Rename', renamePrompt: 'Enter new preset name:', cannotRenameBuiltin: 'Built-in presets cannot be renamed',
    center: 'Center', top: 'Top', left: 'Left', delPresetMsg: 'Are you sure to delete this preset?', presetNamePrompt: 'Enter new preset name:', customThemeDefault: 'New Custom Theme', blank: 'Blank Preset', importSuffix: 'Import',
    saveConfirmMsg: 'Overwrite the current preset?', stop: 'Stop', noSavePath: 'Please select an output directory first!', timecodeErr: 'Timecode errors detected! Please fix the red inputs.',
    success: 'Success', renderSuccess: 'Video exported successfully!', error: 'Error',
    showLog: 'Show Details & Logs ▼', hideLog: 'Hide Details & Logs ▲', taskInfo: 'Tasks', encoder: 'Encoder', initEngine: '[System] WebGL Core Ready', ffmpegFrame: '>> [FFmpeg] Encoding frame {frame}...',
    progColor: 'Progress Color', trackBg: 'Track Bg Color', padding: 'Padding', trackHeight: 'Track Height', 
    borderWidth: 'Border Width', borderColor: 'Border Color', sepWidth: 'Separator Width', sepColor: 'Separator Color', mascotScale: 'Mascot Scale',
    textSize: 'Text Size', textColor: 'Text Color', textWeight: 'Font Weight', font: 'Font Family',
    normal: 'Normal', bold: 'Bold', heavy: 'Heavy', sysFont: 'System Font', localFonts: 'Local Fonts',
    rendering: 'Rendering Video...', progress: 'Progress', remain: 'ETA', cancel: 'Cancel', ok: 'OK',
    fhd: '1920x1080 (FHD)', qhd: '2560x1440 (2K)', uhd: '3840x2160 (4K)',
    uw1080: '2560x1080 (Ultrawide)', uw1440: '3440x1440 (Ultrawide)',
    green: 'MP4 (Green Screen)', alpha: 'MOV (Transparent Bg)', screen169: 'Standard (16:9)', screen219: 'Ultrawide (21:9)'
  }
};
const t = (k: string) => dict[locale.value][k] || k;

// ====== 主题模式调度 ======
const themeMode = ref('auto');
const isDark = ref(false);
const updateDarkMode = () => {
  if (themeMode.value === 'auto') { isDark.value = window.matchMedia('(prefers-color-scheme: dark)').matches; } 
  else { isDark.value = themeMode.value === 'dark'; }
};
const toggleThemeMode = () => {
  const modes = ['auto', 'light', 'dark'];
  themeMode.value = modes[(modes.indexOf(themeMode.value) + 1) % 3];
  localStorage.setItem('timelineCraftThemeMode', themeMode.value);
  updateDarkMode();
};

// ====== 视频核心元数据 ======
const chapters = ref([{ id: 1, time: '00:00:00:00', title: '开头' }, { id: 2, time: '00:01:35:00', title: '第一章' }]);
const fps = ref(60); 
const duration = ref(300); const durationInput = ref('00:05:00:00');
const videoResolution = ref('1920'); 
const videoFormat = ref('mp4_green');
const savePath = ref(''); 
const currentTime = ref(0); 
const currentTimeInput = ref('00:00:00:00');

// ====== 播放引擎状态 ======
const isPlaying = ref(false); const isDragging = ref(false); let wasPlaying = false;
const isRendering = ref(false); const renderProgress = ref(0); const renderEta = ref('--:--');
const showRenderLogs = ref(false);
const currentFrameLog = ref('');

// ====== 系统本地目录与字体映射 ======
const appDirs = ref({ presets: '', output: '', fonts: '' });
const customFonts = ref<any[]>([]);

// 唤起后端指令打开外部浏览器访问 GitHub (带容错捕获)
async function openGithub() { 
  try {
    await invoke('open_github'); 
  } catch (e) {
    cAlert(t('error'), '唤起浏览器失败: ' + String(e));
  }
}

// 通知系统打开本地的 fonts 目录
async function openFontsDir() { await invoke('open_fonts_dir'); }

// 读取并动态加载本地的 ttf/otf 字体到网页 DOM 中
async function loadLocalFontsDir() {
  try {
    const fonts: {name: string, bytes: number[]}[] = await invoke('load_local_fonts');
    const newFonts = [];
    for (let f of fonts) {
        // 清洗名称，确保 CSS 解析不报错
        const fontName = `CF_${f.name.replace(/[^a-zA-Z0-9]/g, '')}`;
        const u8arr = new Uint8Array(f.bytes);
        const blob = new Blob([u8arr]);
        const url = URL.createObjectURL(blob);
        const face = new FontFace(fontName, `url(${url})`);
        await face.load();
        document.fonts.add(face);
        newFonts.push({ label: f.name, value: `"${fontName}"` });
    }
    customFonts.value = newFonts;
  } catch (e) { console.error("加载本地字体失败", e); }
}

// 🎨 主题样式底层配置文件
const currentThemeConfig = ref({ progressColor: '#D5C4EC', trackColor: '#FFFFFF', separatorColor: '#909399', textColor: '#303133', trackHeight: 40, textSize: 16, textWeight: 'bold', fontFamily: 'system-ui', padding: 0, trackBorderWidth: 0, trackBorderColor: '#000000', separatorWidth: 2, showMascot: false, mascotImage: '', mascotScale: 1.0, mascotPosition: 'center' });

// 系统的内置模板
const defaultTemplates = [
  { id: 'builtin-1', name: '雪羽 (无边距默认)', config: JSON.parse(JSON.stringify(currentThemeConfig.value)) },
  { id: 'builtin-2', name: '极客暗夜 (扁平风)', config: { progressColor: '#00A896', trackColor: '#1A1C1E', separatorColor: '#6B778C', textColor: '#F2F5F8', trackHeight: 40, textSize: 14, textWeight: 'normal', fontFamily: 'system-ui', padding: 0, trackBorderWidth: 0, trackBorderColor: '#000000', separatorWidth: 2, showMascot: false, mascotImage: '', mascotScale: 1.0, mascotPosition: 'center' } }
];

const allThemes = ref<any[]>([]); const currentThemeId = ref('');
const builtInThemes = computed(() => allThemes.value.filter(t => t.id.startsWith('builtin-')));
const customThemes = computed(() => allThemes.value.filter(t => !t.id.startsWith('builtin-')));

// 预设选择器状态
const isPresetOpen = ref(false);
const currentThemeName = computed(() => { const target = allThemes.value.find(t => t.id === currentThemeId.value); return target ? target.name : ''; });

// 🚀 自绘工程参数下拉框状态 (消除原生组件)
const isResOpen = ref(false);
const isFpsOpen = ref(false);
const isFormatOpen = ref(false);

const currentResLabel = computed(() => {
  const r = videoResolution.value;
  if(r === '1920') return t('fhd');
  if(r === '2560') return t('qhd');
  if(r === '3840') return t('uhd');
  if(r === '2560_uw') return t('uw1080');
  if(r === '3440_uw') return t('uw1440');
  return r;
});
const currentFormatLabel = computed(() => videoFormat.value === 'mp4_green' ? t('green') : t('alpha'));

function selectRes(val: string) { videoResolution.value = val; isResOpen.value = false; }
function selectFps(val: number | string) { fps.value = Number(val); isFpsOpen.value = false; }
function selectFormat(val: string) { videoFormat.value = val; isFormatOpen.value = false; }

// 自绘样式精调下拉框状态
const isWeightOpen = ref(false);
const isFontOpen = ref(false);
const isMascotPosOpen = ref(false);

const currentWeightLabel = computed(() => {
    const w = currentThemeConfig.value.textWeight;
    if(w === 'normal') return t('normal');
    if(w === 'bold') return t('bold');
    if(w === '900') return t('heavy');
    return w;
});
const currentFontLabel = computed(() => {
    const f = currentThemeConfig.value.fontFamily;
    if(f === 'system-ui') return t('sysFont');
    const found = customFonts.value.find(x => x.value === f);
    return found ? found.label : t('sysFont');
});
const currentMascotPosLabel = computed(() => {
    const p = currentThemeConfig.value.mascotPosition;
    if(p === 'top') return t('top');
    if(p === 'left') return t('left');
    return t('center');
});

function selectWeight(val: string) { currentThemeConfig.value.textWeight = val; isWeightOpen.value = false; }
function selectFont(val: string) { currentThemeConfig.value.fontFamily = val; isFontOpen.value = false; }
function selectMascotPos(val: string) { currentThemeConfig.value.mascotPosition = val; isMascotPosOpen.value = false; }

// ====== 自定义模拟弹窗系统 ======
const customDialog = ref({ show: false, type: 'alert', title: '', message: '', input: '', resolve: null as any });
function cAlert(title: string, message: string) { return new Promise(resolve => customDialog.value = { show: true, type: 'alert', title, message, input: '', resolve }); }
function cConfirm(title: string, message: string) { return new Promise(resolve => customDialog.value = { show: true, type: 'confirm', title, message, input: '', resolve }); }
function cPrompt(title: string, message: string, defaultInput = '') { return new Promise(resolve => customDialog.value = { show: true, type: 'prompt', title, message, input: defaultInput, resolve }); }
function resolveDialog(isOk: boolean) { customDialog.value.show = false; customDialog.value.resolve(customDialog.value.type === 'prompt' ? (isOk ? customDialog.value.input : null) : isOk); }

// 📁 读取本地硬盘里的 JSON 预设文件
async function loadPresetsFromDisk() {
    try {
        const jsons: string[] = await invoke('read_presets', { dir: appDirs.value.presets });
        let loaded = jsons.map(j => {
            try { return JSON.parse(j); } catch(e) { return null; }
        }).filter(p => p && p.id && p.config);
        
        // 如果系统第一次运行，把系统默认模板写入硬盘
        if (loaded.length === 0) {
            loaded = JSON.parse(JSON.stringify(defaultTemplates));
            for (let t of loaded) {
                await invoke('write_file_rust', { 
                    path: `${appDirs.value.presets}/${t.id}.json`, 
                    content: JSON.stringify(t, null, 2) 
                });
            }
        }
        allThemes.value = loaded;
        
        const lastId = localStorage.getItem('lastThemeId');
        if (lastId && allThemes.value.some(t => t.id === lastId)) { 
            currentThemeId.value = lastId; 
        } else { 
            currentThemeId.value = allThemes.value[0].id; 
        }
        onThemeSelect();
    } catch(e) { console.error(e); }
}

// ====== 生命周期函数 ======
onMounted(async () => {
  const savedLocale = localStorage.getItem('timelineCraftLocale'); if (savedLocale) locale.value = savedLocale;
  const savedMode = localStorage.getItem('timelineCraftThemeMode'); if (savedMode) themeMode.value = savedMode;
  window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', updateDarkMode);
  updateDarkMode();
  
  try {
      // 唤起 Rust 请求系统的绝对路径
      appDirs.value = await invoke('init_app_dirs');
      savePath.value = appDirs.value.output; // 强制默认将输出目录定位在程序所在文件夹下
      await loadLocalFontsDir(); // 挂载本地字体
      await loadPresetsFromDisk(); // 挂载本地预设
  } catch(e) { console.error(e); }
});

// 主题切换映射
function onThemeSelect() { const target = allThemes.value.find(t => t.id === currentThemeId.value); if (target) currentThemeConfig.value = JSON.parse(JSON.stringify(target.config)); localStorage.setItem('lastThemeId', currentThemeId.value); }
function selectTheme(id: string) { currentThemeId.value = id; onThemeSelect(); isPresetOpen.value = false; }

// 添加新主题并固化到本地 JSON
async function addTheme() { 
  const name = await cPrompt(t('add'), t('presetNamePrompt'), t('customThemeDefault')); 
  if (name) { 
      const nt = { id: 'custom-' + Date.now(), name, config: JSON.parse(JSON.stringify(currentThemeConfig.value)) }; 
      allThemes.value.push(nt); 
      currentThemeId.value = nt.id; 
      await invoke('write_file_rust', { path: `${appDirs.value.presets}/${nt.id}.json`, content: JSON.stringify(nt, null, 2) });
      localStorage.setItem('lastThemeId', currentThemeId.value);
  } 
}

// 安全重命名主题预设
async function renameTheme() {
  const target = allThemes.value.find(t => t.id === currentThemeId.value);
  if (target) {
    if (target.id.startsWith('builtin-')) {
       return cAlert(t('error'), t('cannotRenameBuiltin'));
    }
    const newName = await cPrompt(t('rename'), t('renamePrompt'), target.name);
    if (newName && (newName as string).trim() !== '') {
       target.name = (newName as string).trim();
       await invoke('write_file_rust', { path: `${appDirs.value.presets}/${target.id}.json`, content: JSON.stringify(target, null, 2) });
       // 更新完成后不需要修改 ID，直接刷新即可
    }
  }
}

// 覆盖保存当前主题
async function saveTheme() { 
  const target = allThemes.value.find(t => t.id === currentThemeId.value); 
  if (target) { 
    const ok = await cConfirm(t('savePreset'), t('saveConfirmMsg'));
    if (!ok) return;
    target.config = JSON.parse(JSON.stringify(currentThemeConfig.value)); 
    await invoke('write_file_rust', { path: `${appDirs.value.presets}/${target.id}.json`, content: JSON.stringify(target, null, 2) });
    await cAlert(t('success'), `${t('savePreset')}: ${target.name}`); 
  } 
}

// 删除预设（前端和物理硬盘双重删除）
async function deleteTheme() { 
  const ok = await cConfirm(t('del'), t('delPresetMsg'));
  if (!ok) return; 
  const targetId = currentThemeId.value;
  allThemes.value = allThemes.value.filter(t => t.id !== targetId); 
  try { await invoke('delete_preset_file', { path: `${appDirs.value.presets}/${targetId}.json` }); } catch(e) {}
  
  if (allThemes.value.length === 0) { 
      const fb = { id: 'fallback-' + Date.now(), name: t('blank'), config: JSON.parse(JSON.stringify(defaultTemplates[0].config)) };
      allThemes.value.push(fb);
      await invoke('write_file_rust', { path: `${appDirs.value.presets}/${fb.id}.json`, content: JSON.stringify(fb, null, 2) });
  } 
  currentThemeId.value = allThemes.value[0].id; onThemeSelect(); 
}

// 导出系统预设：通过 Tauri 调用原生系统的 Save 弹窗，防止浏览器拦截
async function exportTheme() { 
  const theme = allThemes.value.find(t => t.id === currentThemeId.value); 
  if(!theme) return; 
  try {
     const filePath = await save({
         defaultPath: `${appDirs.value.presets}/Theme_${theme.name}.json`,
         filters: [{ name: 'JSON', extensions: ['json'] }]
     });
     if (filePath) {
         await invoke('write_file_rust', { path: filePath, content: JSON.stringify(theme, null, 2) });
         cAlert(t('success'), '预设文件导出成功！');
     }
  } catch(e) { cAlert(t('error'), '导出被取消或失败'); }
}

// 导入外部预设
async function importThemeTrigger() {
    try {
        const selected = await open({
            defaultPath: appDirs.value.presets,
            filters: [{ name: 'JSON', extensions: ['json'] }],
            multiple: false
        });
        if (selected) {
            const content = await invoke('read_file_rust', { path: selected as string });
            const parsed = JSON.parse(content as string);
            if (parsed.config) {
                parsed.id = 'custom-' + Date.now();
                parsed.name = parsed.name + ` (${t('importSuffix')})`;
                allThemes.value.push(parsed);
                currentThemeId.value = parsed.id;
                // 导入的预设顺便落盘固化
                await invoke('write_file_rust', { path: `${appDirs.value.presets}/${parsed.id}.json`, content: JSON.stringify(parsed, null, 2) });
                onThemeSelect();
                cAlert(t('success'), '预设导入成功！');
            }
        }
    } catch(err) { cAlert(t('error'), '导入解析失败或已取消！'); }
}

// 播放物理引擎：使用 RequestAnimationFrame 和 DeltaTime 处理丝滑重绘
function onCurrentTimeChange() { const secs = timecodeToSeconds(currentTimeInput.value, Number(fps.value)); if (!isNaN(secs)) { currentTime.value = Math.min(Math.max(0, secs), duration.value); } }
function onDragStart() { wasPlaying = isPlaying.value; isPlaying.value = false; isDragging.value = true; }
function onDragEnd() { isDragging.value = false; if (wasPlaying) togglePlay(); }
function togglePlay() { isPlaying.value = !isPlaying.value; if (isPlaying.value) { if (currentTime.value >= duration.value) currentTime.value = 0; requestAnimationFrame(playLoop); } }

let lastFrameTime = 0;
function playLoop(now: number) { 
  if (isPlaying.value && !isDragging.value) { 
    if (lastFrameTime === 0) lastFrameTime = now;
    let dt = (now - lastFrameTime) / 1000;
    // 护城河：防止网页挂入后台导致的增量过大，让滑块瞬间飞出去
    if (dt > 0.1) dt = 1 / Number(fps.value);
    lastFrameTime = now;
    
    currentTime.value += dt; 
    if (currentTime.value >= duration.value) {
       currentTime.value = duration.value;
       isPlaying.value = false;
       lastFrameTime = 0;
    } else {
       requestAnimationFrame(playLoop); 
    }
  } 
}

// 验证时码的有效性
function isChapterValid(timeStr: string, index: number) { const regex = /^\d{2}:\d{2}:\d{2}:\d{2}$/; if (!regex.test(timeStr)) return false; const secs = timecodeToSeconds(timeStr, Number(fps.value)); if (secs > duration.value) return false; if (index > 0) { const prevSecs = timecodeToSeconds(chapters.value[index - 1].time, Number(fps.value)); if (secs <= prevSecs) return false; } return true; }
const hasChapterError = computed(() => { return chapters.value.some((ch, i) => !isChapterValid(ch.time, i)); });

// 选择视频的保存输出路径
async function selectFolder() { 
  try { 
      const s = await open({ directory: true, defaultPath: savePath.value || appDirs.value.output }); 
      if (s) savePath.value = s as string; 
  } catch (e) { 
      // 容错：如果原生 dialog 被阻拦，给出手动输入口
      const fb = await cPrompt('Path', 'Please paste absolute path:', savePath.value); 
      if(fb !== null) savePath.value = fb as string; 
  } 
}

// 游标图片的 Base64 转录系统
const mascotFile = ref(); const triggerMascotUpload = () => { if(currentThemeConfig.value.showMascot) mascotFile.value.click(); }; 
const onMascotUpload = (e: any) => { 
  const f = e.target.files[0]; 
  if (f) {
    const reader = new FileReader();
    reader.onload = (ev) => { currentThemeConfig.value.mascotImage = ev.target?.result as string; };
    reader.readAsDataURL(f);
  } 
};

// 时长变动同步
const onDurationChange = () => { duration.value = timecodeToSeconds(durationInput.value, Number(fps.value)); };
// 时码输入框的双向数据绑定
watch(currentTime, v => { 
  if (!isDragging.value && isPlaying.value) { currentTimeInput.value = formatTimecode(v, Number(fps.value)); } 
  else if (!isDragging.value && !isPlaying.value) { currentTimeInput.value = formatTimecode(v, Number(fps.value)); } 
});
watch(isPlaying, v => { if (!v) lastFrameTime = 0; });

function addChapter() { chapters.value.push({ id: Date.now(), time: '00:00:00:00', title: t('newChapter') }); }

// 🧨 最终压制与合成的核弹发射按钮
async function handleGenerate() {
  if (!savePath.value) return cAlert(t('stop'), t('noSavePath'));
  if(hasChapterError.value) return cAlert(t('stop'), t('timecodeErr'));

  try {
    isRendering.value = true; 
    currentFrameLog.value = '';
    const fpsVal = Number(fps.value); const durationVal = Number(duration.value); const total = Math.floor(durationVal * fpsVal); 
    const outName = videoFormat.value === 'mp4_green' ? '\\export.mp4' : '\\export.mov';
    
    // 强制防黑边协议：宽度和高度必须被 2 整数整除
    let renderWidth = Math.round(parseInt(videoResolution.value));
    let renderHeight = Math.round(currentThemeConfig.value.trackHeight);
    if (renderWidth % 2 !== 0) renderWidth += 1;
    if (renderHeight % 2 !== 0) renderHeight += 1;

    // 唤起底层 Rust 的 FFmpeg 渲染管道
    await invoke('start_ffmpeg_render', { 
        ffmpegPath: "ffmpeg.exe", 
        fps: fpsVal, 
        width: renderWidth, 
        height: renderHeight, 
        outputPath: savePath.value + outName, 
        format: videoFormat.value, 
        hardwareAccel: "cpu"
    });
    
    // 初始化 WebGL 二维离屏引擎
    const engine = new RenderEngine(renderWidth, renderHeight); engine.initWebGL();
    const chapterData = chapters.value.map((c, i) => {
        const s = timecodeToSeconds(c.time, fpsVal); const e = i < chapters.value.length - 1 ? timecodeToSeconds(chapters.value[i+1].time, fpsVal) : durationVal;
        return { title: c.title, startX: (s / durationVal) * renderWidth, width: ((e - s) / durationVal) * renderWidth };
    });
    
    // 等待游标图片的完美加载，杜绝渲染死锁
    let mImg: HTMLImageElement | null = null;
    if (currentThemeConfig.value.showMascot && currentThemeConfig.value.mascotImage) { 
        await new Promise((resolve) => {
            mImg = new Image();
            mImg.onload = () => resolve(true);
            mImg.onerror = () => resolve(false); 
            mImg.src = currentThemeConfig.value.mascotImage; 
        });
    }
    
    const start = performance.now();
    
    // 每一帧像素的高频重绘与注入循环
    for (let i = 0; i < total; i++) {
       const buf = await engine.renderFrame({ progress: i/total, config: currentThemeConfig.value, chapterData, mascotImg: mImg, format: videoFormat.value });
       
       await invoke('send_frame', { frameData: new Uint8Array(buf) });

       // 限制界面日志输出频率 (每 10 帧刷一次 UI，防止主线程卡顿)
       if (i % 10 === 0) {
         renderProgress.value = (i / total) * 100;
         const elap = performance.now() - start; const left = Math.ceil(((total - i) * (elap / (i+1))) / 1000); renderEta.value = `${Math.floor(left/60)}m${left%60}s`;
         currentFrameLog.value = t('ffmpegFrame').replace('{frame}', String(i));
       }
    }
    await invoke('finish_render'); isRendering.value = false; cAlert(t('success'), t('renderSuccess'));
  } catch (e) { 
    isRendering.value = false; 
    cAlert(t('error'), String(e)); 
  }
}
</script>

<style scoped>
:global(html), :global(body) { margin: 0 !important; padding: 0 !important; overflow: hidden !important; height: 100vh; }
.tc-container { font-family: -apple-system, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif; background-color: #f2f5f8; height: 100vh; max-height: 100vh; overflow: hidden; color: #2c3e50; padding: 15px 20px; box-sizing: border-box; display: flex; flex-direction: column; transition: background-color 0.3s; }

.tc-top-header { max-width: 1200px; width: 100%; margin: 0 auto 10px; display: flex; align-items: center; justify-content: space-between; flex-shrink: 0; }
.tc-logo { display: flex; align-items: center; gap: 8px; white-space: nowrap; }
.tc-title { font-size: 18px; font-weight: 900; color: #172b4d; transition: color 0.3s; }
.tc-subtitle { font-size: 12px; color: #6b778c; margin-left: 4px; }
.tc-author { font-size: 11px; font-weight: bold; color: #00A896; margin-left: 8px; background: rgba(0, 168, 150, 0.1); padding: 3px 8px; border-radius: 6px; }

.tc-top-actions { display: flex; align-items: center; gap: 8px; flex-shrink: 0; }

.tc-flat-btn { background-color: transparent; border: none !important; outline: none !important; box-shadow: none !important; border-radius: 6px; display: flex; align-items: center; justify-content: center; cursor: pointer; transition: background-color 0.2s, color 0.2s; color: #6b778c; }
.tc-flat-btn:hover { background-color: rgba(0,0,0,0.05); color: #172b4d; }

.tc-top-actions .tc-flat-btn { padding: 6px 12px; font-size: 13px; font-weight: bold; gap: 6px; text-decoration: none; white-space: nowrap; }
.tc-icon-only { width: 32px; height: 32px; flex-shrink: 0; }
.tc-del-btn { width: 32px; height: 32px; flex-shrink: 0; color: #f56c6c; background-color: #fef0f0; }
.tc-del-btn:hover { background-color: #fbc4c4; color: #f56c6c; }

.w-full { width: 100%; } .flex-1 { flex: 1; } .flex-col { display: flex; flex-direction: column; } .mt-3 { margin-top: 12px; } .text-center { text-align: center; }
.font-mono { font-family: 'Consolas', monospace; }

.tc-input, .tc-select { background-color: #f0f2f5; border: 1px solid transparent !important; box-shadow: none !important; border-radius: 6px; height: 32px; padding: 0 10px; font-size: 13px; color: #344563; outline: none !important; transition: all 0.2s ease; box-sizing: border-box; }
.tc-input:focus, .tc-select:focus { background-color: #ffffff; border-color: #00A896 !important; box-shadow: 0 0 0 3px rgba(0, 168, 150, 0.15) !important; }
.tc-input.is-error { border-color: #F56C6C !important; color: #F56C6C; background-color: #fef0f0; box-shadow: 0 0 0 3px rgba(245, 108, 108, 0.15) !important; } 

.tc-custom-select-wrapper { position: relative; }
.tc-custom-select-display { background-color: #f0f2f5; border-radius: 6px; height: 32px; padding: 0 10px; font-size: 13px; font-weight: normal; color: #344563; display: flex; align-items: center; justify-content: space-between; cursor: pointer; transition: 0.2s; border: 1px solid transparent; box-sizing: border-box;}
.tc-custom-select-display:hover { background-color: #e4e7eb; }
.tc-custom-select-display.is-open { background-color: #fff; border-color: #00A896; box-shadow: 0 0 0 3px rgba(0, 168, 150, 0.15); }
.tc-custom-select-backdrop { position: fixed; inset: 0; z-index: 90; }
.tc-custom-select-menu { position: absolute; top: calc(100% + 4px); left: 0; width: 100%; background: #fff; border-radius: 8px; box-shadow: 0 8px 24px rgba(0,0,0,0.1); border: 1px solid #ebecf0; z-index: 100; max-height: 250px; overflow-y: auto; padding: 6px 0; }
.tc-custom-select-menu.is-up { top: auto; bottom: calc(100% + 4px); box-shadow: 0 -8px 24px rgba(0,0,0,0.1); }
.tc-custom-select-optgroup { padding: 6px 12px; font-size: 11px; font-weight: bold; color: #a5adba; }
.tc-custom-select-option { padding: 8px 12px; font-size: 13px; color: #344563; cursor: pointer; transition: 0.1s; }
.tc-custom-select-option:hover { background: #f4f5f7; color: #172b4d; }
.tc-custom-select-option.is-active { background: rgba(0, 168, 150, 0.08); color: #00A896; font-weight: bold; }

.tc-time-input-inline { background: transparent; border: none !important; color: #00A896; font-family: inherit; font-size: inherit; font-weight: bold; width: 95px; text-align: center; outline: none !important; box-shadow: none !important; padding: 0; cursor: text; white-space: nowrap; transition: 0.2s; border-radius: 4px; }
.tc-time-input-inline:focus { background: #fff; box-shadow: 0 0 0 3px rgba(0, 168, 150, 0.15) !important; }

.tc-btn-dashed { border: 1px dashed #c1c7d0 !important; background: transparent; font-weight: 600; height: 38px; color: #6b778c; width: 100%; border-radius: 6px; cursor: pointer; transition: 0.2s; display: inline-flex; align-items: center; justify-content: center;}
.tc-btn-dashed:hover { border-color: #00A896 !important; color: #00A896; background: rgba(0, 168, 150, 0.05); }

.tc-btn-mega { width: 100%; height: 46px; background: #00A896; color: #fff; font-size: 15px; font-weight: bold; border: none !important; outline: none !important; box-shadow: none !important; border-radius: 8px; cursor: pointer; display: flex; align-items: center; justify-content: center; gap: 8px; transition: 0.3s; margin-top: 15px; flex-shrink: 0; }
.tc-btn-mega:hover { background: #008f7f; }

.tc-preview-section { max-width: 1200px; width: 100%; margin: 0 auto 15px; padding: 20px 25px; background: #ffffff; border-radius: 12px; border: none; box-shadow: 0 2px 10px rgba(0,0,0,0.02); box-sizing: border-box; flex-shrink: 0; transition: 0.3s; overflow: visible; }
.tc-canvas-wrapper { width: 100%; background-color: transparent; border-radius: 4px; display: flex; align-items: center; justify-content: center; padding: 35px 0 10px 0; margin-bottom: 15px; overflow: visible; }
.tc-preview-canvas { filter: drop-shadow(0 0 1px rgba(0, 0, 0, 0.2)); transition: filter 0.3s; }

.tc-playback-bar { display: flex; align-items: center; gap: 15px; }
.tc-btn-play { display: flex; align-items: center; gap: 6px; height: 32px; padding: 0 16px; border: none !important; outline: none !important; border-radius: 6px; background: #00A896; color: #fff; font-weight: bold; font-size: 13px; cursor: pointer; flex-shrink: 0; transition: 0.2s; box-shadow: none !important;}
.tc-btn-play:not(.is-playing) { background: #f0f2f5; color: #344563; }
.tc-slider-wrap { flex: 1; display: flex; align-items: center; }
.tc-range-slider { -webkit-appearance: none; width: 100%; height: 6px; background: #e1e4e8; border-radius: 3px; outline: none; transition: none !important; }
.tc-range-slider::-webkit-slider-thumb { -webkit-appearance: none; width: 16px; height: 16px; border-radius: 50%; background: #fff; border: 3px solid #00A896; cursor: pointer; box-shadow: none; transition: none !important; }
.tc-timecode-box { background: #f0f2f5; padding: 4px 12px; border-radius: 6px; border: none; font-family: monospace; font-size: 14px; font-weight: bold; color: #344563; display: flex; align-items: center; gap: 8px; white-space: nowrap; transition: 0.3s; }
.tc-time-sep { color: #a5adba; }
.tc-time-total { color: #344563; transition: 0.3s; }

.tc-toolbar { max-width: 1200px; width: 100%; margin: 0 auto 15px; display: flex; align-items: center; justify-content: space-between; gap: 10px; background: #ffffff; padding: 10px 15px; border-radius: 12px; border: none; box-shadow: 0 2px 10px rgba(0,0,0,0.02); box-sizing: border-box; flex-shrink: 0; transition: 0.3s; overflow: visible; }
.tc-toolbar-item { display: flex; align-items: center; gap: 6px; flex-shrink: 1; white-space: nowrap; }
.tc-label { font-size: 12px; font-weight: bold; color: #6b778c; white-space: nowrap; }
.tc-path-group { display: flex; gap: 6px; width: 100%; }

.tc-workspace { display: flex; gap: 20px; max-width: 1200px; width: 100%; margin: 0 auto; flex: 1; min-height: 0; box-sizing: border-box; }
.tc-card { background: #ffffff; border-radius: 12px; border: none; box-shadow: 0 2px 10px rgba(0,0,0,0.02); display: flex; height: 100%; overflow: hidden; transition: 0.3s; }
.panel-left { flex: 4.5; } .panel-right { flex: 5.5; }

.tc-card-header { padding: 12px 20px; border-bottom: 1px solid #f4f5f7; background: #fafbfc; flex-shrink: 0; display: flex; align-items: center; justify-content: space-between; transition: 0.3s;}
.tc-card-title { font-size: 14px; font-weight: 800; color: #172b4d; display: flex; align-items: center; justify-content: space-between; width: 100%; transition: 0.3s; }
.tc-card-title .tc-sub { font-size: 11px; color: #a5adba; font-weight: 600; letter-spacing: 1px; }

.tc-chapter-hint-flat { display: flex; align-items: center; gap: 6px; font-size: 11px; font-weight: bold; color: #6b778c; background: #f0f2f5; padding: 4px 10px; border-radius: 6px; white-space: nowrap; transition: 0.3s; }
.tc-chapter-hint-flat .dot { width: 6px; height: 6px; border-radius: 50%; background: #00A896; }
.tc-chapter-hint-flat.is-error { color: #f56c6c; background: #fef0f0; }
.tc-chapter-hint-flat.is-error .dot { background: #f56c6c; animation: pulse 1s infinite; }
@keyframes pulse { 0% { opacity: 1; } 50% { opacity: 0.4; } 100% { opacity: 1; } }

.tc-card-body { padding: 15px 20px; flex: 1; overflow: hidden; display: flex; flex-direction: column; }
.tc-card-footer { padding: 15px 20px; background: #fafbfc; border-top: 1px solid #f4f5f7; flex-shrink: 0; transition: 0.3s; }

.tc-table-header { display: flex; font-size: 12px; font-weight: bold; color: #a5adba; padding-bottom: 8px; border-bottom: 1px solid #ebecf0; margin-bottom: 10px; flex-shrink: 0; transition: 0.3s; }
.col-idx { width: 30px; text-align: center; } .col-time { width: 100px; } .col-title { flex: 1; padding: 0 10px; } .col-act { width: 32px; }
.tc-chapter-list { flex: 1; overflow-y: auto; min-height: 0; padding-right: 5px; }
.tc-chapter-list::-webkit-scrollbar { width: 6px; }
.tc-chapter-list::-webkit-scrollbar-thumb { background: #dfe1e6; border-radius: 3px; }
.tc-chapter-row { display: flex; align-items: center; margin-bottom: 8px; }
.tc-chapter-row .col-idx { font-size: 12px; font-weight: bold; color: #b3bac5; transition: 0.3s; }

.overflow-y { overflow-y: auto; min-height: 0; }
.overflow-y::-webkit-scrollbar { width: 6px; }
.overflow-y::-webkit-scrollbar-thumb { background: #dfe1e6; border-radius: 3px; }

.tc-preset-bar { display: flex; align-items: center; gap: 10px; margin-bottom: 15px; justify-content: flex-start; }
.tc-theme-actions { display: flex; align-items: center; gap: 8px; padding-left: 10px; flex-shrink: 0; transition: 0.3s; border-left: 1px solid #ebecf0; }
.tc-action-link { font-size: 12px; color: #00A896; font-weight: bold; cursor: pointer; padding: 4px; transition: 0.2s; white-space: nowrap; }
.tc-action-link:hover { opacity: 0.7; }
.tc-action-link.danger { color: #f56c6c; }

.tc-switch { position: relative; width: 40px; height: 20px; display: inline-block; flex-shrink: 0; }
.tc-switch input { opacity: 0; width: 0; height: 0; }
.tc-switch .slider { position: absolute; cursor: pointer; inset: 0; background-color: #dfe1e6; transition: .3s; border-radius: 20px; }
.tc-switch .slider:before { position: absolute; content: ""; height: 14px; width: 14px; left: 3px; bottom: 3px; background-color: white; transition: .3s; border-radius: 50%; }
.tc-switch input:checked + .slider { background-color: #00A896; }
.tc-switch input:checked + .slider:before { transform: translateX(20px); }
.tc-divider { height: 1px; background: #ebecf0; margin: 15px 0; flex-shrink: 0; transition: 0.3s; }

.tc-settings-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 15px 25px; padding-bottom: 10px; }
.tc-cell { display: flex; flex-direction: column; gap: 6px; }
.tc-cell-head { display: flex; justify-content: space-between; align-items: center; font-size: 12px; font-weight: bold; color: #344563; white-space: nowrap; transition: 0.3s; }
.tc-cell-head .tc-val { color: #a5adba; font-family: monospace; font-weight: normal; }

.tc-color-box { display: flex; align-items: center; gap: 8px; background: #f0f2f5; padding: 4px 8px; border-radius: 6px; font-family: monospace; font-size: 12px; color: #6b778c; border: none; transition: 0.3s;}
.tc-color-box input[type="color"] { width: 16px; height: 16px; border: none; padding: 0; background: transparent; cursor: pointer; }
.tc-color-box input[type="color"]::-webkit-color-swatch-wrapper { padding: 0; }
.tc-color-box input[type="color"]::-webkit-color-swatch { border: 1px solid transparent; border-radius: 4px; }

.tc-slider-native { -webkit-appearance: none; width: 100%; height: 4px; background: #e1e4e8; border-radius: 2px; outline: none; transition: 0.3s; }
.tc-slider-native::-webkit-slider-thumb { -webkit-appearance: none; width: 12px; height: 12px; border-radius: 50%; background: #fff; border: 2px solid #00A896; cursor: pointer; }

.tc-overlay { position: fixed; inset: 0; background: rgba(255,255,255,0.85); display: flex; align-items: center; justify-content: center; z-index: 9999; backdrop-filter: blur(8px); transition: 0.3s; }

.render-modal { background: #fff; width: 460px; padding: 30px; border-radius: 12px; box-shadow: 0 10px 40px rgba(0,0,0,0.1); border: none; text-align: center; transition: 0.3s; }
.modal-title { font-size: 18px; font-weight: 800; color: #172b4d; margin: 0 0 15px; transition: 0.3s; }
.tc-progress-wrap { height: 8px; background: #ebecf0; border-radius: 4px; overflow: hidden; margin-bottom: 15px; transition: 0.3s; }
.tc-progress-bar { height: 100%; background: #00A896; transition: width 0.2s linear; }
.modal-stats { display: flex; justify-content: space-between; font-size: 12px; font-weight: bold; color: #6b778c; margin-bottom: 15px; transition: 0.3s; }
.tc-log-toggle { font-size: 12px; color: #00A896; cursor: pointer; font-weight: bold; user-select: none; transition: 0.2s; }
.tc-log-toggle:hover { opacity: 0.7; }
.tc-log-panel { margin-top: 15px; background: #1c2128; border-radius: 8px; padding: 15px; text-align: left; font-family: monospace; font-size: 12px; color: #a5adba; max-height: 120px; overflow-y: auto; }
.log-info-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 8px; margin-bottom: 12px; border-bottom: 1px solid #2c333f; padding-bottom: 10px; }
.log-info-item span { color: #e1e4e8; font-weight: bold; margin-right: 4px; }
.log-line { margin-top: 4px; }
.log-hl { color: #00A896; font-weight: bold; }

.dialog-modal { background: #fff; width: 380px; padding: 25px; border-radius: 12px; box-shadow: 0 15px 50px rgba(0,0,0,0.15); border: none; display: flex; flex-direction: column; gap: 15px; transition: 0.3s; }
.dialog-header { font-size: 16px; font-weight: bold; color: #172b4d; transition: 0.3s; }
.dialog-body { font-size: 14px; color: #606266; line-height: 1.5; transition: 0.3s; }
.dialog-footer { display: flex; justify-content: flex-end; gap: 10px; margin-top: 10px; }
.dialog-btn-cancel { background: #f0f2f5; color: #344563; border: none; padding: 0 20px; height: 34px; border-radius: 6px; cursor: pointer; font-weight: bold; transition: 0.2s; font-size: 13px; }
.dialog-btn-cancel:hover { background: #e4e7eb; }
.dialog-btn-confirm { background: #00A896; color: #fff; border: none; padding: 0 20px; height: 34px; border-radius: 6px; cursor: pointer; font-weight: bold; transition: 0.2s; font-size: 13px; }
.dialog-btn-confirm:hover { background: #008f7f; }

.dark-theme { background-color: #0f1115 !important; color: #b3bac5 !important; }
.dark-theme .tc-preview-section, .dark-theme .tc-toolbar, .dark-theme .tc-card { background: #1c2128 !important; border-color: transparent !important; box-shadow: 0 2px 10px rgba(0,0,0,0.2) !important; }
.dark-theme .tc-card-header, .dark-theme .tc-card-footer { background: #161a1f !important; border-color: #2c333f !important; }
.dark-theme .tc-title { color: #e1e4e8 !important; }
.dark-theme .tc-card-title { color: #e1e4e8 !important; }
.dark-theme .tc-input, .dark-theme .tc-select, .dark-theme .tc-flat-btn { background-color: #22272e !important; color: #adbac7 !important; border-color: transparent !important; }
.dark-theme .tc-select { background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 24 24' fill='none' stroke='%23a5adba' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3E%3Cpolyline points='6 9 12 15 18 9'/%3E%3C/svg%3E") !important; }
.dark-theme .tc-select option { background-color: #22272e !important; color: #adbac7 !important; }
.dark-theme .tc-select optgroup { background-color: #1c2128 !important; color: #e1e4e8 !important; }
.dark-theme .tc-input:focus, .dark-theme .tc-select:focus { background-color: #2d333b !important; outline: 1px solid #00A896 !important; box-shadow: 0 0 0 3px rgba(0, 168, 150, 0.25) !important; }
.dark-theme .tc-time-input-inline:focus { background-color: #1c2128 !important; border-color: #00A896 !important; box-shadow: 0 0 0 3px rgba(0, 168, 150, 0.25) !important; outline: none !important; }
.dark-theme .tc-flat-btn:hover { background-color: rgba(255,255,255,0.05) !important; color: #e1e4e8 !important; }
.dark-theme .tc-del-btn { background-color: #4b1a1f !important; color: #f56c6c !important; }
.dark-theme .tc-del-btn:hover { background-color: #5c1e24 !important; }
.dark-theme .tc-timecode-box { background: #22272e !important; }
.dark-theme .tc-time-total { color: #a5adba !important; }
.dark-theme .tc-table-header { border-color: #2c333f !important; }
.dark-theme .tc-chapter-row { border-color: #2c333f !important; }
.dark-theme .tc-chapter-row .col-idx { color: #6b778c !important; }
.dark-theme .tc-chapter-hint-flat { background: #22272e !important; color: #a5adba !important; }
.dark-theme .tc-btn-play:not(.is-playing) { background: #22272e !important; color: #e1e4e8 !important; }
.dark-theme .tc-btn-dashed { border-color: #4c566a !important; color: #a5adba !important; }
.dark-theme .tc-btn-dashed:hover { border-color: #00A896 !important; color: #00A896 !important; background: transparent !important; }
.dark-theme .tc-color-box { background: #22272e !important; color: #a5adba !important; }
.dark-theme .tc-slider-native, .dark-theme .tc-range-slider { background: #2c333f !important; }
.dark-theme .tc-modal, .dark-theme .dialog-modal { background: #1c2128 !important; border-color: transparent !important; }
.dark-theme .modal-title, .dark-theme .dialog-header { color: #e1e4e8 !important; }
.dark-theme .dialog-body, .dark-theme .modal-stats { color: #b3bac5 !important; }
.dark-theme .tc-overlay { background: rgba(15, 17, 21, 0.85) !important; }
.dark-theme .tc-divider, .dark-theme .tc-theme-actions { border-color: #2c333f !important; }
.dark-theme .tc-cell-head { color: #e1e4e8 !important; }
.dark-theme .tc-preview-canvas { filter: drop-shadow(0 0 1px rgba(255, 255, 255, 0.3)); }

.dark-theme .tc-custom-select-display { background-color: #22272e; color: #adbac7; border-color: transparent; }
.dark-theme .tc-custom-select-display:hover { background-color: #2d333b; }
.dark-theme .tc-custom-select-display.is-open { background-color: #1c2128; border-color: #00A896; box-shadow: 0 0 0 3px rgba(0, 168, 150, 0.25); }
.dark-theme .tc-custom-select-menu { background: #1c2128; border-color: #2c333f; box-shadow: 0 8px 24px rgba(0,0,0,0.4); }
.dark-theme .tc-custom-select-menu.is-up { box-shadow: 0 -8px 24px rgba(0,0,0,0.4); }

.dark-theme .tc-custom-select-optgroup { color: #6b778c; }
.dark-theme .tc-custom-select-option { color: #adbac7; }
.dark-theme .tc-custom-select-option:hover { background: #2d333b; color: #e1e4e8; }
.dark-theme .tc-custom-select-option.is-active { background: rgba(0, 168, 150, 0.15); color: #00A896; }
.dark-theme .dialog-btn-cancel { background: #22272e; color: #adbac7; }
.dark-theme .dialog-btn-cancel:hover { background: #2d333b; }
</style>