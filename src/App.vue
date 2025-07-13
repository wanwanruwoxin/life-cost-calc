<script setup lang="ts">
const text = ref("");
const price = ref<number>(0);
const frequency = ref<1 | 2 | 3 | 4>();
const priceCalcResult = ref<any>({
  monthly: 0,
  yearly: 0,
});

const priceCalc = () => {
  switch (frequency.value) {
    case 1:
      priceCalcResult.value.monthly = price.value * 30;
      priceCalcResult.value.yearly = price.value * 365;
      break;
    case 2:
      priceCalcResult.value.monthly = price.value * 4;
      priceCalcResult.value.yearly = price.value * 52;
      break;
    case 3:
      priceCalcResult.value.monthly = price.value;
      priceCalcResult.value.yearly = price.value * 12;
      break;
    case 4:
      priceCalcResult.value.monthly = price.value / 12;
      priceCalcResult.value.yearly = price.value;
      break;
    default:
      break;
  }
};
</script>

<template>
  <main class="container">
    <QLayout view="hHh Lpr lFf">
      <QHeader flex="~ content-center justify-center">
        <p>Life Cost Calculator</p>
      </QHeader>
      <QPageContainer>
        <QPage w-full h-full flex="~ items-center justify-center">
          <div w-100 h-100 flex="~ col" border="2px solid red" p-2>
            <div mb-2>
              <QInput v-model="text" label="说明" />
              <QInput v-model.number="price" type="number" label="金额" />
              <div flex>
                <QRadio v-model="frequency" :val="1" label="每天" />
                <QRadio v-model="frequency" :val="2" label="每周" />
                <QRadio v-model="frequency" :val="3" label="每月" />
                <QRadio v-model="frequency" :val="4" label="每年" />
              </div>
            </div>

            <QBtn color="primary" label="开始计算" @click="priceCalc" />
          </div>

          <QCard>
            <QCardSection>
              <div>数据概览</div>
            </QCardSection>
            <QCardSection>
              <div>每月支出: {{ priceCalcResult.monthly }} 元</div>
              <div>每年支出: {{ priceCalcResult.yearly }} 元</div>
            </QCardSection>
          </QCard>
        </QPage>
      </QPageContainer>
    </QLayout>
  </main>
</template>

<style scoped></style>
