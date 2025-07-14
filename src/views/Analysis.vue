<script setup lang="ts">
import { ref, computed } from "vue";
import type { ExpenseRecord } from "../types";

// 模拟数据
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
  },
  {
    id: '3',
    description: '午餐',
    price: 35,
    frequency: 'daily',
    monthlyPrice: 1050,
    yearlyPrice: 12600,
    createdAt: new Date('2024-01-08')
  },
  {
    id: '4',
    description: '电影票',
    price: 50,
    frequency: 'weekly',
    monthlyPrice: 200,
    yearlyPrice: 2400,
    createdAt: new Date('2024-01-05')
  }
]);

const formatCurrency = (amount: number) => {
  return new Intl.NumberFormat("zh-CN", {
    style: "currency",
    currency: "CNY",
  }).format(amount);
};

// 计算统计数据
const totalMonthly = computed(() => {
  return records.value.reduce((sum, record) => sum + record.monthlyPrice, 0);
});

const totalYearly = computed(() => {
  return records.value.reduce((sum, record) => sum + record.yearlyPrice, 0);
});

const averagePerRecord = computed(() => {
  return records.value.length > 0 ? totalMonthly.value / records.value.length : 0;
});

// 按频率分组统计
const frequencyStats = computed(() => {
  const stats = records.value.reduce((acc, record) => {
    if (!acc[record.frequency]) {
      acc[record.frequency] = { count: 0, total: 0 };
    }
    acc[record.frequency].count++;
    acc[record.frequency].total += record.monthlyPrice;
    return acc;
  }, {} as Record<string, { count: number; total: number }>);
  
  return Object.entries(stats).map(([frequency, data]) => ({
    frequency,
    count: data.count,
    total: data.total,
    percentage: (data.total / totalMonthly.value * 100).toFixed(1)
  }));
});

// 最大支出项目
const topExpense = computed(() => {
  return records.value.reduce((max, record) => 
    record.monthlyPrice > max.monthlyPrice ? record : max
  , records.value[0] || null);
});

// 频率标签映射
const getFrequencyLabel = (frequency: string) => {
  const labels: Record<string, string> = {
    daily: '每天',
    weekly: '每周', 
    monthly: '每月',
    yearly: '每年'
  };
  return labels[frequency] || frequency;
};

// 频率颜色映射
const getFrequencyColor = (frequency: string) => {
  const colors: Record<string, string> = {
    daily: 'orange',
    weekly: 'blue',
    monthly: 'green',
    yearly: 'purple'
  };
  return colors[frequency] || 'grey';
};
</script>

<template>
  <QPage class="bg-gradient-to-br from-gray-50 to-blue-100 min-h-screen p-4 pb-20">
    <div class="w-full max-w-md mx-auto">
      <!-- 页面标题 -->
      <div class="mb-8">
        <h1 class="text-3xl font-bold text-gray-800 mb-2">数据分析</h1>
        <p class="text-gray-600">深入了解您的支出模式和趋势</p>
      </div>
      
      <!-- 总览统计 -->
      <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
        <QCard class="rounded-4 shadow-lg backdrop-blur-sm bg-white/95">
          <QCardSection class="bg-gradient-to-br from-blue-50 to-indigo-100 rounded-t-4">
            <div class="flex items-center">
              <QIcon name="account_balance_wallet" size="md" class="mr-4 text-indigo-500" />
              <div>
                <div class="text-sm text-gray-600">月总支出</div>
                <div class="text-2xl font-bold text-gray-800">{{ formatCurrency(totalMonthly) }}</div>
              </div>
            </div>
          </QCardSection>
        </QCard>
        
        <QCard class="rounded-4 shadow-lg backdrop-blur-sm bg-white/95">
          <QCardSection class="bg-gradient-to-br from-green-50 to-emerald-100 rounded-t-4">
            <div class="flex items-center">
              <QIcon name="trending_up" size="md" class="mr-4 text-emerald-500" />
              <div>
                <div class="text-sm text-gray-600">年总支出</div>
                <div class="text-2xl font-bold text-gray-800">{{ formatCurrency(totalYearly) }}</div>
              </div>
            </div>
          </QCardSection>
        </QCard>
        
        <QCard class="rounded-4 shadow-lg backdrop-blur-sm bg-white/95">
          <QCardSection class="bg-gradient-to-br from-purple-50 to-pink-100 rounded-t-4">
            <div class="flex items-center">
              <QIcon name="calculate" size="md" class="mr-4 text-purple-500" />
              <div>
                <div class="text-sm text-gray-600">平均每项</div>
                <div class="text-2xl font-bold text-gray-800">{{ formatCurrency(averagePerRecord) }}</div>
              </div>
            </div>
          </QCardSection>
        </QCard>
      </div>
      
      <div class="space-y-6">
        <!-- 频率分析 -->
        <QCard class="rounded-4 shadow-lg backdrop-blur-sm bg-white/95">
          <QCardSection class="bg-gradient-to-br from-blue-50 to-indigo-50 rounded-t-4 border-b border-indigo-100">
            <div class="text-lg font-medium flex items-center">
              <QIcon name="pie_chart" class="mr-2" />
              频率分析
            </div>
          </QCardSection>
          
          <QCardSection>
            <div class="space-y-4">
              <div 
                v-for="stat in frequencyStats" 
                :key="stat.frequency"
                class="flex items-center justify-between p-3 bg-gray-50 rounded-2"
              >
                <div class="flex items-center">
                  <QBadge 
                    :color="getFrequencyColor(stat.frequency)"
                    :label="getFrequencyLabel(stat.frequency)"
                    class="mr-3"
                  />
                  <span class="text-gray-700">{{ stat.count }} 项</span>
                </div>
                <div class="text-right">
                  <div class="font-semibold text-gray-800">{{ formatCurrency(stat.total) }}</div>
                  <div class="text-sm text-gray-500">{{ stat.percentage }}%</div>
                </div>
              </div>
            </div>
          </QCardSection>
        </QCard>
        
        <!-- 最大支出项 -->
        <QCard class="rounded-4 shadow-lg backdrop-blur-sm bg-white/95">
          <QCardSection class="bg-gradient-to-br from-orange-50 to-red-50 rounded-t-4 border-b border-orange-100">
            <div class="text-lg font-medium flex items-center">
              <QIcon name="trending_up" class="mr-2" />
              最大支出项
            </div>
          </QCardSection>
          
          <QCardSection v-if="topExpense">
            <div class="text-center">
              <QIcon name="star" size="3rem" class="text-orange-500 mb-4" />
              <h3 class="text-xl font-semibold text-gray-800 mb-2">{{ topExpense.description }}</h3>
              <div class="space-y-2">
                <div class="flex justify-between">
                  <span class="text-gray-600">单次金额：</span>
                  <span class="font-semibold">{{ formatCurrency(topExpense.price) }}</span>
                </div>
                <div class="flex justify-between">
                  <span class="text-gray-600">月支出：</span>
                  <span class="font-semibold text-orange-600">{{ formatCurrency(topExpense.monthlyPrice) }}</span>
                </div>
                <div class="flex justify-between">
                  <span class="text-gray-600">年支出：</span>
                  <span class="font-semibold text-red-600">{{ formatCurrency(topExpense.yearlyPrice) }}</span>
                </div>
                <div class="flex justify-between">
                  <span class="text-gray-600">频率：</span>
                  <QBadge 
                    :color="getFrequencyColor(topExpense.frequency)"
                    :label="getFrequencyLabel(topExpense.frequency)"
                  />
                </div>
              </div>
            </div>
          </QCardSection>
        </QCard>
      </div>
      
      <!-- 支出记录详情 -->
      <QCard class="rounded-4 shadow-lg backdrop-blur-sm bg-white/95 mt-8">
        <QCardSection class="bg-gradient-to-br from-gray-50 to-slate-50 rounded-t-4 border-b border-gray-100">
          <div class="text-lg font-medium flex items-center">
            <QIcon name="list" class="mr-2" />
            支出明细
          </div>
        </QCardSection>
        
        <QCardSection class="p-0">
          <div class="space-y-3 p-4">
            <div 
              v-for="record in records" 
              :key="record.id"
              class="bg-gray-50 rounded-2 p-3 border border-gray-200"
            >
              <div class="flex items-center justify-between mb-2">
                <h4 class="font-medium text-gray-800">{{ record.description }}</h4>
                <QBadge 
                  :color="getFrequencyColor(record.frequency)"
                  :label="getFrequencyLabel(record.frequency)"
                />
              </div>
              <div class="grid grid-cols-3 gap-2 text-xs">
                <div class="text-center">
                  <div class="text-gray-500">单次</div>
                  <div class="font-semibold text-gray-800">{{ formatCurrency(record.price) }}</div>
                </div>
                <div class="text-center">
                  <div class="text-gray-500">月支出</div>
                  <div class="font-semibold text-gray-800">{{ formatCurrency(record.monthlyPrice) }}</div>
                </div>
                <div class="text-center">
                  <div class="text-gray-500">年支出</div>
                  <div class="font-semibold text-gray-800">{{ formatCurrency(record.yearlyPrice) }}</div>
                </div>
              </div>
            </div>
          </div>
        </QCardSection>
      </QCard>
    </div>
  </QPage>
</template>

<style scoped>
/* 表格样式优化 */
table {
  border-collapse: collapse;
}

th, td {
  border: none;
}

tr:hover {
  background-color: #f9fafb;
}
</style>