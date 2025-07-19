<script setup lang="ts">
</script>

<template>
  <QLayout view="lHh Lpr lFf" class="mx-auto bg-white min-h-screen">
    <!-- 头部导航栏 -->
    <QHeader
      class="bg-gradient-to-r from-yellow-200 to-yellow-100 text-gray-800 shadow-lg h-60px"
    >
      <QToolbar class="h-full flex items-center justify-center">
        <div class="text-lg font-bold text-gray-700">羊羊的记账本</div>
      </QToolbar>
    </QHeader>

    <!-- 主内容区域 -->
    <QPageContainer class="bg-gradient-to-br from-gray-50 to-blue-100">
      <QScrollArea class="h-[calc(100vh-120px)] w-full max-w-sm mx-auto sm:max-w-md md:max-w-lg">
        <QPage class="bg-gradient-to-br from-gray-50 to-blue-100 h-full p-4">
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
</template>

<style scoped>
/* 样式已迁移到UnoCSS类中 */
</style>
