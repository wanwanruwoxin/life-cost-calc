<script setup lang="ts">
import type { FrequencyType, PriceCalcResult } from "./types";
import { FREQUENCY_CONFIGS, FREQUENCY_OPTIONS } from "./types";

const text = ref("");
const price = ref<number>(0);
const frequency = ref<FrequencyType>();
const priceCalcResult = ref<PriceCalcResult>({
  monthly: 0,
  yearly: 0,
});

const showResult = ref(false);

const priceCalc = () => {
  if (!price.value || !frequency.value) {
    return;
  }

  const config = FREQUENCY_CONFIGS[frequency.value];
  priceCalcResult.value.monthly = price.value * config.monthlyMultiplier;
  priceCalcResult.value.yearly = price.value * config.yearlyMultiplier;
  showResult.value = true;
};

const formatCurrency = (amount: number) => {
  return new Intl.NumberFormat("zh-CN", {
    style: "currency",
    currency: "CNY",
  }).format(amount);
};

const getFrequencyIcon = (freq: FrequencyType) => {
  return FREQUENCY_CONFIGS[freq]?.icon || "help";
};

const getFrequencyLabel = (freq: FrequencyType) => {
  return FREQUENCY_CONFIGS[freq]?.label || "";
};
</script>

<template>
  <main class="min-h-screen">
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
        </QToolbar>
      </QHeader>

      <QPageContainer
        class="bg-gradient-to-br from-gray-50 to-blue-100 min-h-[calc(100vh-50px)]"
      >
        <QPage class="flex flex-center p-4">
          <div class="w-full max-w-150">
            <!-- 输入表单卡片 -->
            <QCard
              class="rounded-4 shadow-lg backdrop-blur-sm bg-white/95 mb-8"
              flat
              bordered
            >
              <QCardSection
                class="bg-gradient-to-br from-blue-50 to-indigo-50 rounded-t-4 border-b border-indigo-100"
              >
                <div class="text-lg font-medium flex items-center">
                  <QIcon name="edit" class="mr-2" />
                  费用信息
                </div>
              </QCardSection>

              <QCardSection class="pt-0">
                <div class="flex flex-col md:flex-row gap-4">
                  <QInput
                    v-model="text"
                    label="费用说明"
                    outlined
                    class="transition-all duration-300 hover:-translate-y-0.5"
                    prefix-icon="description"
                  />

                  <QInput
                    v-model.number="price"
                    type="number"
                    label="金额"
                    outlined
                    class="transition-all duration-300 hover:-translate-y-0.5"
                    prefix="¥"
                    suffix="元"
                  />
                </div>

                <div
                  class="p-4 bg-gray-50 rounded-3 border border-gray-200 mt-4"
                >
                  <div class="text-sm mb-2 text-gray-500">频率选择</div>
                  <div class="flex flex-wrap gap-3">
                    <QBtn
                      v-for="option in FREQUENCY_OPTIONS"
                      :key="option"
                      :color="frequency === option ? 'primary' : 'grey-3'"
                      :text-color="frequency === option ? 'white' : 'grey-7'"
                      :outline="frequency !== option"
                      :unelevated="frequency === option"
                      class="flex-1 min-w-20 px-4 py-3 rounded-2 transition-all duration-300 font-medium hover:-translate-y-0.5 hover:shadow-md"
                      @click="frequency = option"
                    >
                      <QIcon :name="getFrequencyIcon(option)" class="mr-1" />
                      {{ getFrequencyLabel(option) }}
                    </QBtn>
                  </div>
                </div>

                <div class="flex justify-center mt-5">
                  <QBtn
                    color="primary"
                    size="lg"
                    class="px-8 py-4 rounded-3 text-4 font-semibold transition-all duration-300 bg-gradient-to-br from-indigo-500 to-purple-600 hover:-translate-y-0.5 hover:shadow-lg flex justify-center items-center"
                    @click="priceCalc"
                    :disable="!price || !frequency"
                    unelevated
                  >
                    <QIcon name="calculate" class="mr-2" />
                    开始计算
                  </QBtn>
                </div>
              </QCardSection>
            </QCard>

            <!-- 结果展示卡片 -->
            <Transition name="slide-up" appear>
              <QCard
                v-if="showResult"
                class="rounded-4 shadow-lg backdrop-blur-sm bg-white/95"
                flat
                bordered
              >
                <QCardSection
                  class="bg-gradient-to-br from-blue-50 to-indigo-50 rounded-t-4 border-b border-indigo-100"
                >
                  <div class="text-lg font-medium flex items-center">
                    <QIcon name="analytics" class="mr-2" />
                    数据概览
                  </div>
                </QCardSection>

                <QCardSection class="pt-0">
                  <div class="flex flex-col md:flex-row gap-4">
                    <div
                      class="flex items-center p-5 rounded-3 transition-all duration-300 border border-gray-200 bg-gradient-to-br from-cyan-50 to-blue-100 hover:-translate-y-0.5 hover:shadow-md"
                    >
                      <QIcon
                        name="calendar_month"
                        size="md"
                        class="mr-4 text-indigo-500"
                      />
                      <div class="flex-1">
                        <div class="text-3.5 text-slate-600 mb-1 font-medium">
                          每月支出
                        </div>
                        <div class="text-6 font-bold text-slate-800">
                          {{ formatCurrency(priceCalcResult.monthly) }}
                        </div>
                      </div>
                    </div>

                    <div
                      class="flex items-center p-5 rounded-3 transition-all duration-300 border border-gray-200 bg-gradient-to-br from-purple-50 to-pink-100 hover:-translate-y-0.5 hover:shadow-md"
                    >
                      <QIcon
                        name="calendar_today"
                        size="md"
                        class="mr-4 text-indigo-500"
                      />
                      <div class="flex-1">
                        <div class="text-3.5 text-slate-600 mb-1 font-medium">
                          每年支出
                        </div>
                        <div class="text-6 font-bold text-slate-800">
                          {{ formatCurrency(priceCalcResult.yearly) }}
                        </div>
                      </div>
                    </div>
                  </div>

                  <div
                    v-if="text"
                    class="flex items-center p-3 px-4 bg-gray-50 rounded-2 border-l-4 border-indigo-500 mt-4"
                  >
                    <QIcon name="info" class="mr-1" />
                    <span class="text-gray-500">{{ text }}</span>
                  </div>
                </QCardSection>
              </QCard>
            </Transition>
          </div>
        </QPage>
      </QPageContainer>
    </QLayout>
  </main>
</template>

<style scoped>
/* Vue过渡动画效果 */
.slide-up-enter-active {
  transition: all 0.5s ease;
}

.slide-up-enter-from {
  opacity: 0;
  transform: translateY(30px);
}

.slide-up-enter-to {
  opacity: 1;
  transform: translateY(0);
}
</style>
