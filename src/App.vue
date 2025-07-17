<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { attachConsole } from '@tauri-apps/plugin-log';

let initialized = false;
let detach: (() => void) | null = null;

onMounted(async () => {
  if (!initialized) {
    // 附加控制台日志
    detach = await attachConsole();

    await invoke('initialize_app');
    initialized = true;
  }
});

// 组件卸载时分离控制台
onUnmounted(() => {
  if (detach) {
    detach();
  }
});
</script>

<template>
  <QLayout view="lHh Lpr lFf" class="max-w-sm mx-auto bg-white">
    <!-- 头部导航栏 -->
    <QHeader class="bg-gradient-to-r from-yellow-200 to-yellow-100 text-gray-800 shadow-lg">
      <QToolbar class="px-4 py-3 " flex="~ justify-center">
        <div class="text-lg font-bold text-gray-700">羊羊的记账本</div>
      </QToolbar>
    </QHeader>
    
    <!-- 主内容区域 -->
    <QPageContainer class="bg-gradient-to-br from-gray-50 to-blue-100">
      <QPage class="bg-gradient-to-br from-gray-50 to-blue-100 min-h-screen pb-20">
        <RouterView />
      </QPage>
    </QPageContainer>
    
    <!-- 底部导航栏 -->
    <QFooter class="bg-gradient-to-r from-yellow-100 to-yellow-50 border-t border-yellow-200 shadow-lg relative">    
      <QTabs
        v-model="$route.path"
        class="text-gray-600 bg-transparent"
        active-color="blue"
        indicator-color="transparent"
        align="justify"
      >
        <QTab
          name="/records"
          icon="receipt_long"
          label="明细"
          @click="$router.push('/records')"
          class="min-h-16 py-2"
          :class="{
            'text-yellow-600': $route.path === '/records',
            'text-gray-400': $route.path !== '/records'
          }"
        />
        <QTab
          name="/analysis"
          icon="analytics"
          label="图表"
          @click="$router.push('/analysis')"
          class="min-h-16 py-2"
          :class="{
            'text-yellow-600': $route.path === '/analysis',
            'text-gray-400': $route.path !== '/analysis'
          }"
        />
        <QTab
          name="/expense"
          icon="add_circle"
          label="记账"
          @click="$router.push('/expense')"
          class="min-h-16 py-2 relative"
          :class="{
            'text-yellow-600': $route.path === '/expense',
            'text-gray-400': $route.path !== '/expense'
          }"
        />
        <QTab
          name="/discover"
          icon="explore"
          label="发现"
          @click="$router.push('/discover')"
          class="min-h-16 py-2"
          :class="{
            'text-yellow-600': $route.path === '/discover',
            'text-gray-400': $route.path !== '/discover'
          }"
        />
        <QTab
          name="/profile"
          icon="person"
          label="我的"
          @click="$router.push('/profile')"
          class="min-h-16 py-2"
          :class="{
            'text-yellow-600': $route.path === '/profile',
            'text-gray-400': $route.path !== '/profile'
          }"
        />
      </QTabs>
    </QFooter>
  </QLayout>
</template>

<style scoped></style>
