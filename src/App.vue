<script setup lang="ts">
// App.vue 现在作为主布局组件
</script>

<template>
  <QLayout view="hHh Lpr lFf">
    <QHeader
      class="bg-gradient-to-br from-indigo-500 to-purple-600 text-white"
      elevated
    >
      <QToolbar class="px-8">
        <QIcon name="account_balance_wallet" size="md" class="mr-4" />
        <QToolbarTitle class="text-xl font-bold">
          生活成本计算器
        </QToolbarTitle>
        
        <QSpace />
        
        <QBtnGroup flat class="text-white">
          <QBtn 
            flat 
            icon="home" 
            label="首页" 
            @click="$router.push('/')" 
            :class="{ 'bg-white/20': $route.path === '/' }"
          />
          <QBtn 
            flat 
            icon="calculate" 
            label="计算器" 
            @click="$router.push('/calculator')" 
            :class="{ 'bg-white/20': $route.path === '/calculator' }"
          />
          <QBtn 
            flat 
            icon="list" 
            label="记录" 
            @click="$router.push('/records')" 
            :class="{ 'bg-white/20': $route.path === '/records' }"
          />
          <QBtn 
            flat 
            icon="analytics" 
            label="分析" 
            @click="$router.push('/analysis')" 
            :class="{ 'bg-white/20': $route.path === '/analysis' }"
          />
        </QBtnGroup>
      </QToolbar>
    </QHeader>

    <QPageContainer class="bg-gradient-to-br from-gray-50 to-blue-100 min-h-[calc(100vh-50px)]">
      <RouterView />
    </QPageContainer>
  </QLayout>
</template>

<style scoped></style>
