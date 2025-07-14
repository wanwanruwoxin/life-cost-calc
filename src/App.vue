<script setup lang="ts">
// App.vue 移动端布局组件
</script>

<template>
  <QLayout view="lHh Lpr lFf" class="max-w-sm mx-auto bg-white">
    <!-- 主内容区域 -->
    <QPageContainer class="bg-gradient-to-br from-gray-50 to-blue-100">
      <RouterView />
    </QPageContainer>
    
    <!-- 底部导航栏 -->
    <QFooter class="bg-white border-t border-gray-200 shadow-lg">
      <QTabPanels v-model="$route.path" animated class="bg-transparent">
        <QTabPanel name="/records" class="p-0" />
        <QTabPanel name="/analysis" class="p-0" />
        <QTabPanel name="/calculator" class="p-0" />
        <QTabPanel name="/discover" class="p-0" />
        <QTabPanel name="/profile" class="p-0" />
      </QTabPanels>
      
      <QTabs
        v-model="$route.path"
        class="text-gray-600 bg-white"
        active-color="primary"
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
            'text-primary': $route.path === '/records',
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
            'text-primary': $route.path === '/analysis',
            'text-gray-400': $route.path !== '/analysis'
          }"
        />
        <QTab
          name="/calculator"
          icon="add_circle"
          label="记账"
          @click="$router.push('/calculator')"
          class="min-h-16 py-2 relative"
          :class="{
            'text-primary': $route.path === '/calculator',
            'text-gray-400': $route.path !== '/calculator'
          }"
        />
        <QTab
          name="/discover"
          icon="explore"
          label="发现"
          @click="$router.push('/discover')"
          class="min-h-16 py-2"
          :class="{
            'text-primary': $route.path === '/discover',
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
            'text-primary': $route.path === '/profile',
            'text-gray-400': $route.path !== '/profile'
          }"
        />
      </QTabs>
    </QFooter>
  </QLayout>
</template>

<style scoped></style>
