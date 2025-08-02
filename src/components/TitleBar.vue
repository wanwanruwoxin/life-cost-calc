<template>
  <div class="title-bar" data-tauri-drag-region>
    <!-- 左侧：应用图标和标题 -->
    <div class="title-bar-left">
      <div class="app-icon">
        <QIcon
          name="account_balance_wallet"
          size="sm"
          class="text-yellow-600"
        />
      </div>
      <span class="app-title">羊羊的记账本</span>
    </div>

    <!-- 右侧：窗口控制按钮 -->
    <div class="title-bar-controls">
      <QBtn
        @click="handleMinimize"
        class="control-btn minimize-btn"
        flat
        dense
        round
        size="sm"
      >
        <QIcon name="minimize" size="xs" />
      </QBtn>

      <QBtn
        @click="handleToggleMaximize"
        class="control-btn maximize-btn"
        flat
        dense
        round
        size="sm"
      >
        <QIcon
          :name="isMaximized ? 'fullscreen_exit' : 'fullscreen'"
          size="xs"
        />
      </QBtn>

      <QBtn
        @click="handleClose"
        class="control-btn close-btn"
        flat
        dense
        round
        size="sm"
      >
        <QIcon name="close" size="xs" />
      </QBtn>
    </div>
  </div>
</template>

<script setup lang="ts">
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
const isMaximized = ref(false);

// 最小化窗口
const handleMinimize = async () => {
  try {
    await getCurrentWebviewWindow().minimize();
  } catch (error) {
    console.error("最小化失败:", error);
  }
};

// 最大化/还原窗口
const handleToggleMaximize = async () => {
  try {
    if (isMaximized.value) {
      await getCurrentWebviewWindow().unmaximize();
      isMaximized.value = false;
    } else {
      await getCurrentWebviewWindow().maximize();
      isMaximized.value = true;
    }
  } catch (error) {
    console.error("切换最大化状态失败:", error);
  }
};

// 关闭窗口
const handleClose = async () => {
  try {
    await getCurrentWebviewWindow().close();
  } catch (error) {
    console.error("关闭窗口失败:", error);
  }
};

</script>

<style scoped>
.title-bar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  background: linear-gradient(135deg, #fef3c7 0%, #fde68a 100%);
  border-bottom: 1px solid #f59e0b;
  padding: 0 8px;
  user-select: none;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  z-index: 1000;
}

.title-bar-left {
  display: flex;
  align-items: center;
  gap: 8px;
  pointer-events: none;
}

.app-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 20px;
  height: 20px;
}

.app-title {
  font-size: 13px;
  font-weight: 500;
  color: #92400e;
}

.title-bar-controls {
  display: flex;
  align-items: center;
  gap: 2px;
  pointer-events: auto;
}

.control-btn {
  width: 28px !important;
  height: 28px !important;
  min-width: 28px !important;
  border-radius: 4px !important;
  transition: all 0.2s ease;
}

.minimize-btn:hover {
  background-color: rgba(156, 163, 175, 0.2);
  color: #374151;
}

.maximize-btn:hover {
  background-color: rgba(34, 197, 94, 0.2);
  color: #15803d;
}

.close-btn:hover {
  background-color: rgba(239, 68, 68, 0.2);
  color: #dc2626;
}

.control-btn:active {
  transform: scale(0.95);
}

/* 确保拖拽区域正确工作 */
[data-tauri-drag-region] {
  -webkit-app-region: drag;
}

.title-bar-controls {
  -webkit-app-region: no-drag;
}
</style>
