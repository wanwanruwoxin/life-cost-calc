<script setup lang="ts">
import { db } from "@/utils/database";
import type { Category } from "@/types/database";

const activeTab = ref("expense");
const selectedCategory = ref("");
const amount = ref<number>(0);
const amountString = ref("0");

// 动态分类数据
const expenseCategories = ref<Category[]>([]);
const incomeCategories = ref<Category[]>([]);

// 加载分类数据
const loadCategories = async () => {
  try {
    const [expenses, incomes] = await Promise.all([
      db.categories.getCategories("expense"),
      db.categories.getCategories("income"),
    ]);

    expenseCategories.value = expenses || [];
    incomeCategories.value = incomes || [];
  } catch (error) {
    console.error("Failed to load categories:", error);
  }
};

const selectCategory = (category: any) => {
  selectedCategory.value = category.category_id;
  // 重置金额
  amountString.value = "0";
  amount.value = 0;
};

// 组件挂载时加载分类数据
onMounted(() => {
  loadCategories();
});
</script>

<template>
  <div class="w-full max-w-md">
    <!-- 顶部标签页 -->
    <QCard class="rounded-4 shadow-lg backdrop-blur-sm bg-white/95" flat bordered>
      <QTabs v-model="activeTab" class="text-gray-600" active-color="yellow-600" indicator-color="yellow-600"
        align="justify">
        <QTab name="expense" icon="remove" label="支出" class="text-lg font-medium" />
        <QTab name="income" icon="add" label="收入" class="text-lg font-medium" />
      </QTabs>
    </QCard>

    <!-- 分类选择 -->
    <QCard class="rounded-4 shadow-lg backdrop-blur-sm bg-white/95" flat bordered>
      <QCardSection>
        <div class="grid grid-cols-4 gap-3">
          <div v-for="category in activeTab === 'expense'
            ? expenseCategories
            : incomeCategories" :key="category.category_id" @click="selectCategory(category)"
            class="flex flex-col items-center p-3 rounded-3 cursor-pointer transition-all duration-300 hover:-translate-y-1 hover:shadow-md"
            :class="{
              'bg-yellow-100 border-2 border-yellow-400':
                selectedCategory === category.category_id,
              'bg-gray-50 border border-gray-200':
                selectedCategory !== category.category_id,
            }">
            <QIcon :name="category.icon" size="md" :class="`text-${category.color}-500`" />
            <span class="text-xs mt-1 text-center font-medium">{{
              category.name
              }}</span>
          </div>
        </div>
      </QCardSection>
    </QCard>
  </div>
</template>

<style scoped></style>
