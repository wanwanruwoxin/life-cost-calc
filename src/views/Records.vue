<script setup lang="ts">
import { ref, onMounted, computed } from "vue";
import { db } from "@/utils/database";
import type { ExpenseRecord, Category } from "@/types/database";

const records = ref<ExpenseRecord[]>([]);
const categories = ref<Category[]>([]);
const loading = ref(false);
const selectedType = ref<"all" | "expense" | "income">("all");

// 加载记录数据
const loadRecords = async () => {
  loading.value = true;
  try {
    const filter =
      selectedType.value !== "all"
        ? { record_type: selectedType.value }
        : undefined;
    records.value = await db.records.getRecords(filter);
  } catch (error) {
    console.error("Failed to load records:", error);
  } finally {
    loading.value = false;
  }
};

// 加载分类数据（用于显示分类名称）
const loadCategories = async () => {
  try {
    categories.value = await db.categories.getCategories();
  } catch (error) {
    console.error("Failed to load categories:", error);
  }
};

// 获取分类信息
const getCategoryInfo = (categoryId: string) => {
  return categories.value.find((cat) => cat.category_id === categoryId);
};

const formatCurrency = (amount: string | number) => {
  const num = typeof amount === "string" ? parseFloat(amount) : amount;
  return new Intl.NumberFormat("zh-CN", {
    style: "currency",
    currency: "CNY",
  }).format(num);
};

const formatDate = (dateString: string) => {
  return new Date(dateString).toLocaleDateString("zh-CN", {
    year: "numeric",
    month: "2-digit",
    day: "2-digit",
  });
};

const deleteRecord = async (id: number) => {
  try {
    await db.records.deleteRecord(id);
    await loadRecords(); // 重新加载数据
  } catch (error) {
    console.error("Failed to delete record:", error);
  }
};

// 计算统计数据
const totalExpenses = computed(() => {
  return records.value
    .filter((record) => record.record_type === "expense")
    .reduce((sum, record) => sum + parseFloat(record.amount), 0);
});

const totalIncome = computed(() => {
  return records.value
    .filter((record) => record.record_type === "income")
    .reduce((sum, record) => sum + parseFloat(record.amount), 0);
});

const balance = computed(() => {
  return totalIncome.value - totalExpenses.value;
});

// 筛选类型变化时重新加载数据
const onTypeChange = () => {
  loadRecords();
};

// 组件挂载时加载数据
onMounted(() => {
  loadCategories();
  loadRecords();
});
</script>

<template>
  <div class="w-full max-w-md mx-auto p-4">
    <!-- 页面标题 -->
    <div class="mb-8">
      <h1 class="text-3xl font-bold text-gray-800 mb-2">支出记录</h1>
      <p class="text-gray-600">管理您的所有支出记录</p>
    </div>

    <!-- 筛选器 -->
    <div class="mb-6">
      <QBtnToggle
        v-model="selectedType"
        @update:model-value="onTypeChange"
        toggle-color="primary"
        :options="[
          { label: '全部', value: 'all' },
          { label: '支出', value: 'expense' },
          { label: '收入', value: 'income' },
        ]"
        class="w-full"
      />
    </div>

    <!-- 统计卡片 -->
    <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-8">
      <QCard class="rounded-4 shadow-lg backdrop-blur-sm bg-white/95">
        <QCardSection
          class="bg-gradient-to-br from-red-50 to-red-100 rounded-t-4"
        >
          <div class="flex items-center">
            <QIcon name="trending_down" size="md" class="mr-3 text-red-500" />
            <div>
              <div class="text-sm text-gray-600">总支出</div>
              <div class="text-xl font-bold text-gray-800">
                {{ formatCurrency(totalExpenses) }}
              </div>
            </div>
          </div>
        </QCardSection>
      </QCard>

      <QCard class="rounded-4 shadow-lg backdrop-blur-sm bg-white/95">
        <QCardSection
          class="bg-gradient-to-br from-green-50 to-green-100 rounded-t-4"
        >
          <div class="flex items-center">
            <QIcon name="trending_up" size="md" class="mr-3 text-green-500" />
            <div>
              <div class="text-sm text-gray-600">总收入</div>
              <div class="text-xl font-bold text-gray-800">
                {{ formatCurrency(totalIncome) }}
              </div>
            </div>
          </div>
        </QCardSection>
      </QCard>

      <QCard class="rounded-4 shadow-lg backdrop-blur-sm bg-white/95">
        <QCardSection
          class="bg-gradient-to-br from-blue-50 to-blue-100 rounded-t-4"
        >
          <div class="flex items-center">
            <QIcon
              name="account_balance_wallet"
              size="md"
              class="mr-3 text-blue-500"
            />
            <div>
              <div class="text-sm text-gray-600">余额</div>
              <div
                class="text-xl font-bold"
                :class="{
                  'text-green-600': balance >= 0,
                  'text-red-600': balance < 0,
                }"
              >
                {{ formatCurrency(balance) }}
              </div>
            </div>
          </div>
        </QCardSection>
      </QCard>
    </div>

    <!-- 记录列表 -->
    <QCard class="rounded-4 shadow-lg backdrop-blur-sm bg-white/95">
      <QCardSection
        class="bg-gradient-to-br from-blue-50 to-indigo-50 rounded-t-4 border-b border-indigo-100"
      >
        <div class="text-lg font-medium flex items-center">
          <QIcon name="list_alt" class="mr-2" />
          记录列表
          <QBadge :label="records.length" color="primary" class="ml-2" />
        </div>
      </QCardSection>

      <QCardSection class="pt-0">
        <QInnerLoading :showing="loading">
          <QSpinnerDots size="50px" color="primary" />
        </QInnerLoading>

        <div v-if="!loading && records.length === 0" class="text-center py-12">
          <QIcon name="inbox" size="4rem" class="text-gray-300 mb-4" />
          <p class="text-gray-500">暂无记录</p>
          <QBtn
            @click="$router.push('/expense')"
            color="primary"
            outline
            class="mt-4"
          >
            开始记账
          </QBtn>
        </div>

        <div v-else-if="!loading" class="space-y-4">
          <div
            v-for="record in records"
            :key="record.id"
            class="flex items-center justify-between p-4 bg-gray-50 rounded-3 border border-gray-200 hover:shadow-md transition-all duration-300"
          >
            <div class="flex-1">
              <div class="flex items-center mb-2">
                <div class="flex items-center mr-3">
                  <QIcon
                    :name="
                      getCategoryInfo(record.category_id)?.icon || 'category'
                    "
                    :class="`text-${
                      getCategoryInfo(record.category_id)?.color || 'gray'
                    }-500 mr-2`"
                    size="sm"
                  />
                  <h3 class="text-lg font-medium text-gray-800">
                    {{
                      getCategoryInfo(record.category_id)?.name ||
                      record.category_id
                    }}
                  </h3>
                </div>
                <QBadge
                  :color="
                    record.record_type === 'expense' ? 'negative' : 'positive'
                  "
                  :label="record.record_type === 'expense' ? '支出' : '收入'"
                />
              </div>
              <div
                class="grid grid-cols-1 md:grid-cols-2 gap-4 text-sm text-gray-600"
              >
                <div>
                  <span class="font-medium">金额：</span>
                  <span
                    :class="{
                      'text-red-600': record.record_type === 'expense',
                      'text-green-600': record.record_type === 'income',
                    }"
                  >
                    {{ record.record_type === "expense" ? "-" : "+"
                    }}{{ formatCurrency(record.amount) }}
                  </span>
                </div>
                <div v-if="record.note">
                  <span class="font-medium">备注：</span>
                  {{ record.note }}
                </div>
              </div>
              <div class="text-xs text-gray-400 mt-2">
                记录时间：{{
                  record.created_at ? formatDate(record.created_at) : "未知"
                }}
              </div>
            </div>

            <div class="ml-4">
              <QBtn
                icon="delete"
                color="negative"
                flat
                round
                @click="deleteRecord(record.id!)"
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
        @click="$router.push('/expense')"
      >
        <QTooltip>添加新记录</QTooltip>
      </QBtn>
    </div>
  </div>
</template>

<style scoped>
/* 记录项悬停效果 */
.record-item:hover {
  transform: translateY(-2px);
}
</style>
