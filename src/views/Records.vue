<script setup lang="ts">
import type { ExpenseRecord } from "../types";

const records = ref<ExpenseRecord[]>([
  {
    id: '1',
    description: '咖啡',
    price: 25,
    frequency: 'daily',
    monthlyPrice: 750,
    yearlyPrice: 9000,
    createdAt: new Date('2024-01-15')
  },
  {
    id: '2', 
    description: '健身房会员',
    price: 200,
    frequency: 'monthly',
    monthlyPrice: 200,
    yearlyPrice: 2400,
    createdAt: new Date('2024-01-10')
  }
]);

const formatCurrency = (amount: number) => {
  return new Intl.NumberFormat("zh-CN", {
    style: "currency",
    currency: "CNY",
  }).format(amount);
};

const getFrequencyLabel = (frequency: string) => {
  const labels: Record<string, string> = {
    daily: '每天',
    weekly: '每周', 
    monthly: '每月',
    yearly: '每年'
  };
  return labels[frequency] || frequency;
};

const deleteRecord = (id: string) => {
  const index = records.value.findIndex(record => record.id === id);
  if (index > -1) {
    records.value.splice(index, 1);
  }
};

const totalMonthly = computed(() => {
  return records.value.reduce((sum, record) => sum + record.monthlyPrice, 0);
});

const totalYearly = computed(() => {
  return records.value.reduce((sum, record) => sum + record.yearlyPrice, 0);
});
</script>

<template>
  <QPage class="bg-gradient-to-br from-gray-50 to-blue-100 min-h-screen p-4">
    <div class="w-full max-w-6xl mx-auto">
      <!-- 页面标题 -->
      <div class="mb-8">
        <h1 class="text-3xl font-bold text-gray-800 mb-2">支出记录</h1>
        <p class="text-gray-600">管理您的所有支出记录</p>
      </div>
      
      <!-- 统计卡片 -->
      <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-8">
        <QCard class="rounded-4 shadow-lg backdrop-blur-sm bg-white/95">
          <QCardSection class="bg-gradient-to-br from-cyan-50 to-blue-100 rounded-t-4">
            <div class="flex items-center">
              <QIcon name="calendar_month" size="md" class="mr-4 text-indigo-500" />
              <div>
                <div class="text-sm text-gray-600">总月支出</div>
                <div class="text-2xl font-bold text-gray-800">{{ formatCurrency(totalMonthly) }}</div>
              </div>
            </div>
          </QCardSection>
        </QCard>
        
        <QCard class="rounded-4 shadow-lg backdrop-blur-sm bg-white/95">
          <QCardSection class="bg-gradient-to-br from-purple-50 to-pink-100 rounded-t-4">
            <div class="flex items-center">
              <QIcon name="calendar_today" size="md" class="mr-4 text-indigo-500" />
              <div>
                <div class="text-sm text-gray-600">总年支出</div>
                <div class="text-2xl font-bold text-gray-800">{{ formatCurrency(totalYearly) }}</div>
              </div>
            </div>
          </QCardSection>
        </QCard>
      </div>
      
      <!-- 记录列表 -->
      <QCard class="rounded-4 shadow-lg backdrop-blur-sm bg-white/95">
        <QCardSection class="bg-gradient-to-br from-blue-50 to-indigo-50 rounded-t-4 border-b border-indigo-100">
          <div class="text-lg font-medium flex items-center">
            <QIcon name="list_alt" class="mr-2" />
            支出记录列表
          </div>
        </QCardSection>
        
        <QCardSection class="pt-0">
          <div v-if="records.length === 0" class="text-center py-12">
            <QIcon name="inbox" size="4rem" class="text-gray-300 mb-4" />
            <p class="text-gray-500">暂无记录</p>
          </div>
          
          <div v-else class="space-y-4">
            <div 
              v-for="record in records" 
              :key="record.id"
              class="flex items-center justify-between p-4 bg-gray-50 rounded-3 border border-gray-200 hover:shadow-md transition-all duration-300"
            >
              <div class="flex-1">
                <div class="flex items-center mb-2">
                  <h3 class="text-lg font-medium text-gray-800 mr-3">{{ record.description }}</h3>
                  <QBadge 
                    :color="record.frequency === 'daily' ? 'orange' : record.frequency === 'weekly' ? 'blue' : record.frequency === 'monthly' ? 'green' : 'purple'"
                    :label="getFrequencyLabel(record.frequency)"
                  />
                </div>
                <div class="grid grid-cols-1 md:grid-cols-3 gap-4 text-sm text-gray-600">
                  <div>
                    <span class="font-medium">单次金额：</span>
                    {{ formatCurrency(record.price) }}
                  </div>
                  <div>
                    <span class="font-medium">月支出：</span>
                    {{ formatCurrency(record.monthlyPrice) }}
                  </div>
                  <div>
                    <span class="font-medium">年支出：</span>
                    {{ formatCurrency(record.yearlyPrice) }}
                  </div>
                </div>
                <div class="text-xs text-gray-400 mt-2">
                  创建时间：{{ record.createdAt.toLocaleDateString('zh-CN') }}
                </div>
              </div>
              
              <div class="ml-4">
                <QBtn 
                  icon="delete" 
                  color="negative" 
                  flat 
                  round 
                  @click="deleteRecord(record.id)"
                  class="hover:bg-red-50"
                >
                  <QTooltip>删除记录</QTooltip>
                </QBtn>
              </div>
            </div>
          </div>
        </QCardSection>
      </QCard>
      
      <!-- 添加记录按钮 -->
      <div class="fixed bottom-6 right-6">
        <QBtn 
          icon="add" 
          color="primary" 
          fab 
          class="shadow-lg"
          @click="$router.push('/calculator')"
        >
          <QTooltip>添加新记录</QTooltip>
        </QBtn>
      </div>
    </div>
  </QPage>
</template>

<style scoped>
/* 记录项悬停效果 */
.record-item:hover {
  transform: translateY(-2px);
}
</style>