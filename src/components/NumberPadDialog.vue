<template>
  <div bg-white>
    <div text-2xl text-right p-1 >{{ numberCountStore.countExpression }}</div>
    <!-- 输入框 -->
    <QInput v-model="note" color="teal" filled label="备注" />
  </div>
  <div flex w-full h-15>
    <NumberPadButton label="7" />
    <NumberPadButton label="8" />
    <NumberPadButton label="9" />
    <NumberPadButton label="今天" />
  </div>
  <div flex w-full h-15>
    <NumberPadButton label="4" />
    <NumberPadButton label="5" />
    <NumberPadButton label="6" />
    <NumberPadButton label="+" />
  </div>
  <div flex w-full h-15>
    <NumberPadButton label="1" />
    <NumberPadButton label="2" />
    <NumberPadButton label="3" />
    <NumberPadButton label="-" />
  </div>
  <div flex w-full h-15>
    <NumberPadButton label="." />
    <NumberPadButton label="0" />
    <NumberPadButton label="x" />
    <NumberPadButton :label="numberCountStore.isExpression ? '=' : '完成'"/>
  </div>
</template>

<script setup lang="ts">
import { useNumberCountStore } from '@/store/numberCountStore';

const note = ref("1111");
const numberCountStore = useNumberCountStore();
</script>

<style scoped></style>
