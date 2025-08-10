<script setup lang="ts">
import { ref, onMounted, computed } from "vue";
import { db } from "@/utils/database";
import type { ExpenseRecord, Category } from "@/types/database";
import MonthPickerDialog from "@/components/MonthPickerDialog.vue";

const records = ref<ExpenseRecord[]>([]);
const categories = ref<Category[]>([]);
const loading = ref(false);
const selectedType = ref<"all" | "expense" | "income">("all");

// 月份滚轮选择（使用组件）
const showMonthDialog = ref(false);
const now = new Date();
const selectedYear = ref<number>(now.getFullYear());
const selectedMonth = ref<number>(now.getMonth() + 1); // 1-12

const monthLabel = computed(
  () => `${selectedYear.value}-${String(selectedMonth.value).padStart(2, "0")}`
);

const getMonthDateRange = (year: number, month: number) => {
  const start = new Date(Date.UTC(year, month - 1, 1, 0, 0, 0, 0));
  const end = new Date(Date.UTC(year, month, 0, 23, 59, 59, 999));
  return { start: start.toISOString(), end: end.toISOString() };
};

// 加载记录数据
const loadRecords = async () => {
  loading.value = true;
  try {
    const filter: any = {};
    if (selectedType.value !== "all") {
      filter.record_type = selectedType.value;
    }
    const { start, end } = getMonthDateRange(
      selectedYear.value,
      selectedMonth.value
    );
    filter.start_date = start;
    filter.end_date = end;
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

// 按月份分组记录（YYYY-MM）
const groupedByMonth = computed(() => {
  const groups: Record<string, ExpenseRecord[]> = {};
  for (const rec of records.value) {
    const key = rec.created_at ? rec.created_at.slice(0, 7) : "未知";
    if (!groups[key]) groups[key] = [];
    groups[key].push(rec);
  }
  const sortedKeys = Object.keys(groups).sort((a, b) => b.localeCompare(a));
  return sortedKeys.map((key) => ({ key, items: groups[key] }));
});

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

// 月份选择滚动逻辑已封装进 MonthPickerDialog 组件，此处不再需要滚动事件

const applyMonthFilter = () => {
  showMonthDialog.value = false;
  loadRecords();
};

// 组件挂载时加载数据
onMounted(() => {
  loadCategories();
  loadRecords();
});
</script>

<template>
  <div w-full max-w-md mx-auto>
    <div flex items-center>
      <div flex-grow-1>
        <QBtn
          outline
          color="primary"
          icon="event"
          :label="monthLabel"
          @click="showMonthDialog = true"
        />
      </div>
      <div flex-grow-4 flex justify-evenly items-center>
        <div class="text-center">
          <div class="text-xs text-gray-500 mb-1">收入</div>
          <div class="text-2xl font-extrabold text-blue-600">
            +{{ formatCurrency(totalIncome) }}
          </div>
        </div>
        <div class="text-center">
          <div class="text-xs text-gray-500 mb-1">支出</div>
          <div class="text-2xl font-extrabold text-red-600">
            -{{ formatCurrency(totalExpenses) }}
          </div>
        </div>
      </div>
    </div>

    <MonthPickerDialog
      v-model="showMonthDialog"
      v-model:year="selectedYear"
      v-model:month="selectedMonth"
      @confirm="applyMonthFilter"
    />

    <!-- 记录列表（每条一行，按月分组，组间分割线） -->
    <div class="mt-4 space-y-6">
      <div v-for="group in groupedByMonth" :key="group.key">
        <!-- 月份分割线与标题 -->
        <div class="flex items-center my-2">
          <div class="flex-1 border-t border-gray-200"></div>
          <div class="px-3 text-gray-500 text-sm">{{ group.key }}</div>
          <div class="flex-1 border-t border-gray-200"></div>
        </div>

        <!-- 分组内的记录列表 -->
        <div class="divide-y divide-gray-100 bg-white rounded-3">
          <div
            v-for="record in group.items"
            :key="record.id ?? record.created_at"
            class="flex items-center justify-between py-3 px-2"
          >
            <div class="flex items-center gap-3">
              <QIcon
                :name="getCategoryInfo(record.category_id)?.icon || 'category'"
                :class="`text-${
                  getCategoryInfo(record.category_id)?.color || 'gray'
                }-500`"
                size="md"
              />
              <div>
                <div class="text-base font-medium text-gray-800">
                  {{
                    getCategoryInfo(record.category_id)?.name ||
                    record.category_id
                  }}
                </div>
                <div v-if="record.note" class="text-xs text-gray-400 mt-0.5">
                  {{ record.note }}
                </div>
                <div class="text-xs text-gray-400 mt-0.5">
                  {{
                    record.created_at
                      ? formatDate(record.created_at)
                      : "未知时间"
                  }}
                </div>
              </div>
            </div>

            <div
              class="text-lg font-semibold"
              :class="{
                'text-blue-600': record.record_type === 'income',
                'text-red-600': record.record_type === 'expense',
              }"
            >
              {{ record.record_type === "expense" ? "-" : "+"
              }}{{ formatCurrency(record.amount) }}
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped></style>
