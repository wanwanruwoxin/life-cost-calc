<script setup lang="ts">
import { useRouter } from "vue-router";

const router = useRouter();

const navigateTo = (path: string) => {
  router.push(path);
};
</script>

<template>
  <div class="w-full max-w-4xl p-6">
    <div class="text-center mb-12">
      <QIcon
        name="account_balance_wallet"
        size="4rem"
        class="text-indigo-500 mb-4"
      />
      <h1 class="text-4xl font-bold text-gray-800 mb-4">生活成本计算器</h1>
      <p class="text-lg text-gray-600">
        智能记账，精准分析，让每一笔支出都有意义
      </p>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      <!-- 费用计算卡片 -->
      <QCard
        class="rounded-4 shadow-lg backdrop-blur-sm bg-white/95 cursor-pointer transition-all duration-300 hover:-translate-y-2 hover:shadow-xl"
        @click="navigateTo('/calculator')"
      >
        <QCardSection
          class="bg-gradient-to-br from-blue-50 to-indigo-50 rounded-t-4"
        >
          <div class="flex items-center justify-center mb-4">
            <QIcon name="calculate" size="3rem" class="text-indigo-500" />
          </div>
          <h3 class="text-xl font-semibold text-center text-gray-800">
            费用计算
          </h3>
        </QCardSection>
        <QCardSection>
          <p class="text-gray-600 text-center">
            根据消费频率计算月度和年度支出
          </p>
        </QCardSection>
      </QCard>

      <!-- 记录管理卡片 -->
      <QCard
        class="rounded-4 shadow-lg backdrop-blur-sm bg-white/95 cursor-pointer transition-all duration-300 hover:-translate-y-2 hover:shadow-xl"
        @click="navigateTo('/records')"
      >
        <QCardSection
          class="bg-gradient-to-br from-green-50 to-emerald-50 rounded-t-4"
        >
          <div class="flex items-center justify-center mb-4">
            <QIcon name="list_alt" size="3rem" class="text-emerald-500" />
          </div>
          <h3 class="text-xl font-semibold text-center text-gray-800">
            记录管理
          </h3>
        </QCardSection>
        <QCardSection>
          <p class="text-gray-600 text-center">保存和管理您的支出记录</p>
        </QCardSection>
      </QCard>

      <!-- 数据分析卡片 -->
      <QCard
        class="rounded-4 shadow-lg backdrop-blur-sm bg-white/95 cursor-pointer transition-all duration-300 hover:-translate-y-2 hover:shadow-xl"
        @click="navigateTo('/analysis')"
      >
        <QCardSection
          class="bg-gradient-to-br from-purple-50 to-pink-50 rounded-t-4"
        >
          <div class="flex items-center justify-center mb-4">
            <QIcon name="analytics" size="3rem" class="text-purple-500" />
          </div>
          <h3 class="text-xl font-semibold text-center text-gray-800">
            数据分析
          </h3>
        </QCardSection>
        <QCardSection>
          <p class="text-gray-600 text-center">深入分析支出趋势和模式</p>
        </QCardSection>
      </QCard>
    </div>

    <div class="mt-12 text-center">
      <p class="text-gray-500">开始您的智能记账之旅</p>
    </div>
  </div>
</template>

<style scoped>
/* 卡片悬停动画 */
.q-card:hover {
  transform: translateY(-8px);
}
</style>
