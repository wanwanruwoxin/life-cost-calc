<script setup lang="ts">
import TitleBar from "@/components/TitleBar.vue";

const thumbStyle: any = {
  right: "4px",
  borderRadius: "5px",
  backgroundColor: "#d97706",
  width: "5px",
  opacity: 0.75,
};
</script>

<template>
  <div class="app-container">
    <QLayout view="hHh lpr fFf" flex flex-col class="mx-auto bg-white h-screen">
      <!-- 头部导航栏 -->
      <QHeader><TitleBar /></QHeader>

      <!-- 主内容区域 -->
      <QPageContainer class="flex-1 bg-gradient-to-br from-gray-50 to-blue-100">
        <QScrollArea
          class="w-full max-w-sm mx-auto sm:max-w-md md:max-w-lg"
          :visible="false"
          :thumb-style="thumbStyle"
        >
          <QPage
            class="bg-gradient-to-br from-gray-50 to-blue-100 min-h-full p-4"
          >
            <RouterView />
          </QPage>
        </QScrollArea>
      </QPageContainer>

      <!-- 底部导航栏 -->
      <QFooter
        class="bg-gradient-to-r from-yellow-100 to-yellow-50 border-t border-yellow-200 shadow-lg h-80px"
      >
        <QTabs
          v-model="$route.path"
          class="text-gray-600 bg-transparent h-full"
          active-color="blue"
          indicator-color="transparent"
          align="justify"
        >
          <QTab
            name="/records"
            icon="receipt_long"
            label="明细"
            @click="$router.push('/records')"
            class="h-full flex flex-col items-center justify-center"
            :class="{
              'text-yellow-600': $route.path === '/records',
              'text-gray-400': $route.path !== '/records',
            }"
          />
          <QTab
            name="/analysis"
            icon="analytics"
            label="图表"
            @click="$router.push('/analysis')"
            class="h-full flex flex-col items-center justify-center"
            :class="{
              'text-yellow-600': $route.path === '/analysis',
              'text-gray-400': $route.path !== '/analysis',
            }"
          />
          <QTab
            name="/expense"
            icon="add_circle"
            label="记账"
            @click="$router.push('/expense')"
            class="h-full flex flex-col items-center justify-center"
            :class="{
              'text-yellow-600': $route.path === '/expense',
              'text-gray-400': $route.path !== '/expense',
            }"
          />
          <QTab
            name="/discover"
            icon="explore"
            label="发现"
            @click="$router.push('/discover')"
            class="h-full flex flex-col items-center justify-center"
            :class="{
              'text-yellow-600': $route.path === '/discover',
              'text-gray-400': $route.path !== '/discover',
            }"
          />
          <QTab
            name="/profile"
            icon="person"
            label="我的"
            @click="$router.push('/profile')"
            class="h-full flex flex-col items-center justify-center"
            :class="{
              'text-yellow-600': $route.path === '/profile',
              'text-gray-400': $route.path !== '/profile',
            }"
          />
        </QTabs>
      </QFooter>
    </QLayout>
  </div>
</template>

<style scoped>
/* 确保 QPageContainer 占据剩余空间 */
.q-page-container {
  flex: 1;
  min-height: 0; /* 允许 flex 子元素收缩 */
}

/* 确保 QScrollArea 占据全部可用高度 */
:deep(.q-scrollarea) {
  height: 100% !important;
}

/* 确保 QPage 至少占据全部高度 */
:deep(.q-page) {
  min-height: 100% !important;
}
</style>
