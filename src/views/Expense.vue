<script setup lang="ts">
import { db } from "@/utils/database";
import type { Category, ExpenseRecordData } from "@/types/database";
import { useRouter } from "vue-router";

const activeTab = ref("expense");
const selectedCategory = ref("");
const amount = ref<number>(0);
const showNumberPad = ref(false);

type CompletePayload = { amount: number; note: string };
const router = useRouter();

const handleNumberPadComplete = async (payload: CompletePayload) => {
  amount.value = payload.amount;
  try {
    const recordData: ExpenseRecordData = {
      record_type: activeTab.value as "expense" | "income",
      category_id: selectedCategory.value,
      amount: payload.amount.toString(),
      note: payload.note || null,
    };
    await db.records.createRecord(recordData);
  } catch (error) {
    console.error("Failed to save record:", error);
  } finally {
    showNumberPad.value = false;
    router.push("/records");
  }
};

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
  amount.value = 0;
  // 打开底部弹出数字键盘
  showNumberPad.value = true;
};

// 组件挂载时加载分类数据
onMounted(() => {
  loadCategories();
});
</script>

<template>
  <div class="w-full max-w-md">
    <!-- 顶部标签页 -->
    <QCard
      class="rounded-4 shadow-lg backdrop-blur-sm bg-white/95"
      flat
      bordered
    >
      <QTabs
        v-model="activeTab"
        class="text-gray-600"
        active-color="yellow-600"
        indicator-color="yellow-600"
        align="justify"
      >
        <QTab
          name="expense"
          icon="remove"
          label="支出"
          class="text-lg font-medium"
        />
        <QTab
          name="income"
          icon="add"
          label="收入"
          class="text-lg font-medium"
        />
      </QTabs>
    </QCard>

    <!-- 分类选择 -->
    <QCard
      class="rounded-4 shadow-lg backdrop-blur-sm bg-white/95"
      flat
      bordered
    >
      <QCardSection>
        <div class="grid grid-cols-4 gap-3">
          <div
            v-for="category in activeTab === 'expense'
              ? expenseCategories
              : incomeCategories"
            :key="category.category_id"
            @click="selectCategory(category)"
            class="flex flex-col items-center p-3 rounded-3 cursor-pointer transition-all duration-300 hover:-translate-y-1 hover:shadow-md"
            :class="{
              'bg-yellow-100 border-2 border-yellow-400':
                selectedCategory === category.category_id,
              'bg-gray-50 border border-gray-200':
                selectedCategory !== category.category_id,
            }"
          >
            <QIcon
              :name="category.icon"
              size="md"
              :class="`text-${category.color}-500`"
            />
            <span class="text-xs mt-1 text-center font-medium">{{
              category.name
            }}</span>
          </div>
        </div>
      </QCardSection>
    </QCard>

    <QDialog
      v-model="showNumberPad"
      position="bottom"
      transition-show="slide-up"
      transition-hide="slide-down"
    >
      <QCard class="w-full rounded-t-4 shadow-lg">
        <NumberPadDialog
          v-if="showNumberPad"
          @complete="handleNumberPadComplete"
        />
      </QCard>
    </QDialog>
  </div>
</template>

<style scoped></style>
