<template>
  <div bg-white>
    <div text-2xl text-right p-1>{{ numberCountStore.countExpression }}</div>
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
    <template v-if="numberCountStore.isExpression">
      <NumberPadButton label="=" />
    </template>
    <template v-else>
      <q-btn
        flex-grow-1
        flex-basis-0
        color="yellow-6"
        text-color="white"
        label="完成"
        square
        @click="onComplete"
      />
    </template>
  </div>
</template>

<script setup lang="ts">
import { useNumberCountStore } from "@/store/numberCountStore";

const note = ref("");
const numberCountStore = useNumberCountStore();

const emit = defineEmits<{
  (e: "complete", payload: { amount: number; note: string }): void;
}>();

const onComplete = () => {
  const raw = numberCountStore.countExpression as unknown as string;
  const amount = Number(raw);
  emit("complete", { amount: isNaN(amount) ? 0 : amount, note: note.value });
};
</script>

<style scoped></style>
