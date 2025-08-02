import { WebviewWindow } from '@tauri-apps/api/webviewWindow';

// 获取当前窗口实例的辅助函数
const getCurrentWindow = () => WebviewWindow.getCurrent();

/**
 * 最小化窗口
 */
export async function minimizeWindow(): Promise<void> {
  try {
    await getCurrentWindow().minimize();
  } catch (error) {
    console.error('最小化窗口失败:', error);
    throw error;
  }
}

/**
 * 最大化窗口
 */
export async function maximizeWindow(): Promise<void> {
  try {
    await getCurrentWindow().maximize();
  } catch (error) {
    console.error('最大化窗口失败:', error);
    throw error;
  }
}

/**
 * 还原窗口
 */
export async function unmaximizeWindow(): Promise<void> {
  try {
    await getCurrentWindow().unmaximize();
  } catch (error) {
    console.error('还原窗口失败:', error);
    throw error;
  }
}

/**
 * 关闭窗口
 */
export async function closeWindow(): Promise<void> {
  try {
    await getCurrentWindow().close();
  } catch (error) {
    console.error('关闭窗口失败:', error);
    throw error;
  }
}

/**
 * 检查窗口是否最大化
 */
export async function isWindowMaximized(): Promise<boolean> {
  try {
    return await getCurrentWindow().isMaximized();
  } catch (error) {
    console.error('检查窗口状态失败:', error);
    return false;
  }
}

/**
 * 切换最大化状态
 */
export async function toggleMaximizeWindow(): Promise<boolean> {
  try {
    const maximized = await isWindowMaximized();
    if (maximized) {
      await unmaximizeWindow();
    } else {
      await maximizeWindow();
    }
    return !maximized;
  } catch (error) {
    console.error('切换窗口状态失败:', error);
    throw error;
  }
}

/**
 * 检查是否在Tauri环境中运行
 */
export function isTauriApp(): boolean {
  // 在 Tauri 2.x 中，推荐使用以下方式检测
  if (typeof window === 'undefined') {
    return false;
  }
  
  // 方法1: 检查 __TAURI_INTERNALS__ (Tauri 2.x 新方式)
  if ('__TAURI_INTERNALS__' in window) {
    return true;
  }
  
  // 方法2: 检查传统的 __TAURI__ (向后兼容)
  if ('__TAURI__' in window) {
    return true;
  }
  
  // 方法3: 尝试检测 Tauri API 是否可用
  try {
    return typeof window !== 'undefined' && 
           (window as any).__TAURI_INTERNALS__ !== undefined;
  } catch {
    return false;
  }
}