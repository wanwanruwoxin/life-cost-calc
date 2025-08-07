<script setup lang="ts">
import { ref, computed } from "vue";

const activeTab = ref("expense");
const selectedCategory = ref("");
const amount = ref<number>(0);
const note = ref("");
const amountString = ref("0");

// 计算显示的金额
const displayAmount = computed(() => {
  if (amountString.value === "0") return "0.00";

  // 如果包含小数点
  if (amountString.value.includes(".")) {
    const parts = amountString.value.split(".");
    if (parts[1] && parts[1].length === 1) {
      return amountString.value + "0";
    }
    return amountString.value;
  }

  return amountString.value + ".00";
});

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
  // 重置金额
  amountString.value = "0";
  amount.value = 0;
};

// 数字键盘方法
const inputNumber = (digit: string) => {
  if (amountString.value === "0") {
    amountString.value = digit;
  } else {
    // 限制小数点后最多两位
    if (amountString.value.includes(".")) {
      const parts = amountString.value.split(".");
      if (parts[1] && parts[1].length >= 2) return;
    }
    amountString.value += digit;
  }
  amount.value = parseFloat(amountString.value);
};

const inputDecimal = () => {
  if (!amountString.value.includes(".")) {
    amountString.value += ".";
  }
};

const deleteLastDigit = () => {
  if (amountString.value.length > 1) {
    amountString.value = amountString.value.slice(0, -1);
  } else {
    amountString.value = "0";
  }
  amount.value = parseFloat(amountString.value) || 0;
};

const clearAmount = () => {
  amountString.value = "0";
  amount.value = 0;
};

const addOperation = () => {
  // 简单的加法操作，可以扩展为更复杂的计算器功能
  const currentAmount = parseFloat(amountString.value) || 0;
  amountString.value = (currentAmount + 1).toString();
  amount.value = parseFloat(amountString.value);
};

const subtractOperation = () => {
  // 简单的减法操作
  const currentAmount = parseFloat(amountString.value) || 0;
  if (currentAmount > 0) {
    amountString.value = Math.max(0, currentAmount - 1).toString();
    amount.value = parseFloat(amountString.value);
  }
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
  amountString.value = "0";
  note.value = "";
};
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

    <!-- 数字键盘 - 固定在底部 -->
    <Transition name="slide-up" appear>
      <div
        v-if="selectedCategory"
        class="fixed bottom-0 left-0 right-0 bg-white border-t border-gray-200 shadow-2xl z-50"
      >
        <!-- 金额显示区域 -->
        <div
          class="bg-gradient-to-br from-yellow-50 to-yellow-100 p-4 border-b border-yellow-200"
        >
          <div class="text-center">
            <div class="text-sm text-gray-600 mb-1">输入金额</div>
            <div
              class="text-3xl font-bold text-gray-800 min-h-[3rem] flex items-center justify-center"
            >
              ¥{{ displayAmount }}
            </div>
          </div>
        </div>

        <!-- 备注输入 -->
        <div class="px-4 py-2 bg-gray-50 border-b border-gray-200">
          <QInput
            v-model="note"
            label="备注（可选）"
            outlined
            dense
            maxlength="50"
            class="bg-white"
          />
        </div>

        <!-- 数字键盘 -->
        <div class="p-4 bg-white">
          <div class="grid grid-cols-4 gap-3">
            <!-- 第一行 -->
            <QBtn
              @click="inputNumber('7')"
              class="number-btn"
              unelevated
              size="lg"
            >
              7
            </QBtn>
            <QBtn
              @click="inputNumber('8')"
              class="number-btn"
              unelevated
              size="lg"
            >
              8
            </QBtn>
            <QBtn
              @click="inputNumber('9')"
              class="number-btn"
              unelevated
              size="lg"
            >
              9
            </QBtn>
            <QBtn
              @click="deleteLastDigit"
              class="operation-btn"
              color="red-5"
              unelevated
              size="lg"
            >
              <QIcon name="backspace" />
            </QBtn>

            <!-- 第二行 -->
            <QBtn
              @click="inputNumber('4')"
              class="number-btn"
              unelevated
              size="lg"
            >
              4
            </QBtn>
            <QBtn
              @click="inputNumber('5')"
              class="number-btn"
              unelevated
              size="lg"
            >
              5
            </QBtn>
            <QBtn
              @click="inputNumber('6')"
              class="number-btn"
              unelevated
              size="lg"
            >
              6
            </QBtn>
            <QBtn
              @click="addOperation"
              class="operation-btn"
              color="blue-5"
              unelevated
              size="lg"
            >
              <QIcon name="add" />
            </QBtn>

            <!-- 第三行 -->
            <QBtn
              @click="inputNumber('1')"
              class="number-btn"
              unelevated
              size="lg"
            >
              1
            </QBtn>
            <QBtn
              @click="inputNumber('2')"
              class="number-btn"
              unelevated
              size="lg"
            >
              2
            </QBtn>
            <QBtn
              @click="inputNumber('3')"
              class="number-btn"
              unelevated
              size="lg"
            >
              3
            </QBtn>
            <QBtn
              @click="subtractOperation"
              class="operation-btn"
              color="orange-5"
              unelevated
              size="lg"
            >
              <QIcon name="remove" />
            </QBtn>

            <!-- 第四行 -->
            <QBtn
              @click="inputNumber('0')"
              class="number-btn"
              unelevated
              size="lg"
            >
              0
            </QBtn>
            <QBtn @click="inputDecimal" class="number-btn" unelevated size="lg">
              .
            </QBtn>
            <QBtn
              @click="clearAmount"
              class="operation-btn"
              color="grey-5"
              unelevated
              size="lg"
            >
              <QIcon name="clear" />
            </QBtn>
            <QBtn
              @click="saveRecord"
              :disable="!selectedCategory || !amount"
              class="complete-btn"
              color="yellow-600"
              unelevated
              size="lg"
            >
              <QIcon name="check" />
            </QBtn>
          </div>
        </div>
      </div>
    </Transition>
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

/* 数字键盘按钮样式 */
.number-btn {
  @apply bg-gray-100 text-gray-800 font-semibold rounded-2 min-h-[3.5rem] transition-all duration-200;
}

.number-btn:hover {
  @apply bg-gray-200 transform scale-105;
}

.number-btn:active {
  @apply bg-gray-300 transform scale-95;
}

.operation-btn {
  @apply font-semibold rounded-2 min-h-[3.5rem] transition-all duration-200;
}

.operation-btn:hover {
  @apply transform scale-105;
}

.operation-btn:active {
  @apply transform scale-95;
}

.complete-btn {
  @apply font-semibold rounded-2 min-h-[3.5rem] transition-all duration-200;
}

.complete-btn:hover {
  @apply transform scale-105;
}

.complete-btn:active {
  @apply transform scale-95;
}

/* 从底部滑出动画 */
.slide-up-enter-active {
  transition: all 0.4s cubic-bezier(0.25, 0.46, 0.45, 0.94);
}

.slide-up-leave-active {
  transition: all 0.3s cubic-bezier(0.55, 0.06, 0.68, 0.19);
}

.slide-up-enter-from {
  transform: translateY(100%);
  opacity: 0;
}

.slide-up-leave-to {
  transform: translateY(100%);
  opacity: 0;
}

/* 淡入动画 */
.fade-in-enter-active {
  transition: all 0.5s ease-out;
  transition-delay: 0.2s;
}

.fade-in-leave-active {
  transition: all 0.3s ease-in;
}

.fade-in-enter-from {
  opacity: 0;
  transform: translateY(20px);
}

.fade-in-leave-to {
  opacity: 0;
  transform: translateY(20px);
}

/* 确保主内容区域有足够的底部间距，避免被键盘遮挡 */
.main-content {
  padding-bottom: 400px;
}
</style>
