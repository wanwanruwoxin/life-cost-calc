<script setup lang="ts">
import { ref } from "vue";

const activeTab = ref("expense");
const selectedCategory = ref("");
const amount = ref<number>(0);
const note = ref("");

// 支出分类数据
const expenseCategories = [
  { id: "food", name: "餐饮", icon: "restaurant", color: "orange" },
  { id: "shopping", name: "购物", icon: "shopping_cart", color: "pink" },
  { id: "daily", name: "日用", icon: "home", color: "blue" },
  { id: "transport", name: "交通", icon: "directions_car", color: "green" },
  { id: "vegetables", name: "蔬菜", icon: "eco", color: "green" },
  { id: "fruits", name: "水果", icon: "apple", color: "red" },
  { id: "snacks", name: "零食", icon: "cookie", color: "brown" },
  { id: "sports", name: "运动", icon: "fitness_center", color: "blue" },
  { id: "entertainment", name: "娱乐", icon: "movie", color: "purple" },
  { id: "communication", name: "通讯", icon: "phone", color: "blue" },
  { id: "clothing", name: "服饰", icon: "checkroom", color: "pink" },
  { id: "beauty", name: "美容", icon: "face", color: "pink" },
  { id: "housing", name: "住房", icon: "house", color: "brown" },
  { id: "household", name: "居家", icon: "chair", color: "grey" },
  { id: "children", name: "孩子", icon: "child_care", color: "yellow" },
  { id: "elderly", name: "长辈", icon: "elderly", color: "grey" },
  { id: "social", name: "社交", icon: "group", color: "blue" },
  { id: "travel", name: "旅行", icon: "flight", color: "cyan" },
  { id: "tobacco", name: "烟酒", icon: "local_bar", color: "red" },
  { id: "digital", name: "数码", icon: "devices", color: "blue" },
  { id: "car", name: "汽车", icon: "directions_car", color: "grey" },
  { id: "medical", name: "医疗", icon: "local_hospital", color: "red" },
  { id: "books", name: "书籍", icon: "book", color: "brown" },
  { id: "study", name: "学习", icon: "school", color: "blue" },
  { id: "pets", name: "宠物", icon: "pets", color: "orange" },
  { id: "gift_money", name: "礼金", icon: "card_giftcard", color: "red" },
  { id: "gifts", name: "礼物", icon: "redeem", color: "pink" },
  { id: "office", name: "办公", icon: "work", color: "grey" },
  { id: "repair", name: "维修", icon: "build", color: "orange" },
  { id: "donation", name: "捐赠", icon: "volunteer_activism", color: "green" },
  { id: "lottery", name: "彩票", icon: "casino", color: "yellow" },
  { id: "friends", name: "亲友", icon: "family_restroom", color: "blue" },
  { id: "express", name: "快递", icon: "local_shipping", color: "brown" },
  { id: "settings", name: "设置", icon: "settings", color: "grey" },
];

// 收入分类数据
const incomeCategories = [
  { id: "salary", name: "工资", icon: "work", color: "green" },
  { id: "bonus", name: "奖金", icon: "star", color: "yellow" },
  { id: "investment", name: "投资", icon: "trending_up", color: "blue" },
  { id: "part_time", name: "兼职", icon: "schedule", color: "orange" },
  { id: "gift", name: "礼金", icon: "card_giftcard", color: "red" },
  { id: "other", name: "其他", icon: "more_horiz", color: "grey" },
];

const selectCategory = (category: any) => {
  selectedCategory.value = category.id;
};

const saveRecord = () => {
  if (!selectedCategory.value || !amount.value) {
    return;
  }

  // 这里可以添加保存逻辑
  console.log({
    type: activeTab.value,
    category: selectedCategory.value,
    amount: amount.value,
    note: note.value,
    date: new Date(),
  });

  // 重置表单
  selectedCategory.value = "";
  amount.value = 0;
  note.value = "";
};
</script>

<template>
  <div class="w-full max-w-md mx-auto p-4">
    <!-- 顶部标签页 -->
    <QCard
      class="rounded-4 shadow-lg backdrop-blur-sm bg-white/95 mb-6"
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
      class="rounded-4 shadow-lg backdrop-blur-sm bg-white/95 mb-6"
      flat
      bordered
    >
      <QCardSection
        class="bg-gradient-to-br from-yellow-50 to-yellow-100 rounded-t-4 border-b border-yellow-200"
      >
        <div class="text-lg font-medium flex items-center">
          <QIcon
            :name="
              activeTab === 'expense' ? 'category' : 'account_balance_wallet'
            "
            class="mr-2"
          />
          {{ activeTab === "expense" ? "支出分类" : "收入分类" }}
        </div>
      </QCardSection>

      <QCardSection class="pt-4">
        <div class="grid grid-cols-4 gap-3">
          <div
            v-for="category in activeTab === 'expense'
              ? expenseCategories
              : incomeCategories"
            :key="category.id"
            @click="selectCategory(category)"
            class="flex flex-col items-center p-3 rounded-3 cursor-pointer transition-all duration-300 hover:-translate-y-1 hover:shadow-md"
            :class="{
              'bg-yellow-100 border-2 border-yellow-400':
                selectedCategory === category.id,
              'bg-gray-50 border border-gray-200':
                selectedCategory !== category.id,
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

    <!-- 金额输入 -->
    <QCard
      class="rounded-4 shadow-lg backdrop-blur-sm bg-white/95 mb-6"
      flat
      bordered
    >
      <QCardSection
        class="bg-gradient-to-br from-yellow-50 to-yellow-100 rounded-t-4 border-b border-yellow-200"
      >
        <div class="text-lg font-medium flex items-center">
          <QIcon name="payments" class="mr-2" />
          金额
        </div>
      </QCardSection>

      <QCardSection class="pt-4">
        <QInput
          v-model.number="amount"
          type="number"
          label="请输入金额"
          outlined
          prefix="¥"
          suffix="元"
          class="text-2xl"
          input-class="text-center text-2xl font-bold"
        />

        <QInput
          v-model="note"
          label="备注（可选）"
          outlined
          class="mt-4"
          maxlength="50"
        />
      </QCardSection>
    </QCard>

    <!-- 保存按钮 -->
    <div class="flex justify-center">
      <QBtn
        @click="saveRecord"
        :disable="!selectedCategory || !amount"
        color="yellow-600"
        size="lg"
        class="px-12 py-4 rounded-3 text-lg font-semibold transition-all duration-300 hover:-translate-y-1 hover:shadow-lg"
        unelevated
      >
        <QIcon name="save" class="mr-2" />
        保存记录
      </QBtn>
    </div>
  </div>
</template>

<style scoped>
/* 自定义样式 */
.grid {
  display: grid;
}

.grid-cols-4 {
  grid-template-columns: repeat(4, minmax(0, 1fr));
}

.gap-3 {
  gap: 0.75rem;
}
</style>
