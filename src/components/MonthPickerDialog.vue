<template>
  <QDialog
    v-model="localOpen"
    position="bottom"
    transition-show="slide-up"
    transition-hide="slide-down"
  >
    <QCard class="w-full rounded-t-4">
      <QCardSection class="text-lg font-medium">选择月份</QCardSection>
      <QCardSection>
        <div class="picker-container">
          <div
            class="picker-column"
            :style="{
              height: visibleRows * itemHeight + 'px',
              paddingTop: columnPadding + 'px',
              paddingBottom: columnPadding + 'px',
            }"
            ref="yearColRef"
            @scroll="onYearScroll"
          >
            <div v-for="year in yearList" :key="year" class="picker-item">
              {{ year }}年
            </div>
          </div>
          <div
            class="picker-column"
            :style="{
              height: visibleRows * itemHeight + 'px',
              paddingTop: columnPadding + 'px',
              paddingBottom: columnPadding + 'px',
            }"
            ref="monthColRef"
            @scroll="onMonthScroll"
          >
            <div v-for="m in monthList" :key="m" class="picker-item">
              {{ String(m).padStart(2, "0") }}月
            </div>
          </div>
          <div
            class="picker-indicator"
            :style="{ height: itemHeight + 'px' }"
          />
        </div>
      </QCardSection>
      <div class="flex justify-end gap-2 p-3">
        <QBtn flat color="grey-7" label="取消" @click="close" />
        <QBtn color="primary" label="确定" @click="confirm" />
      </div>
    </QCard>
  </QDialog>
</template>

<script setup lang="ts">
import { computed, nextTick, ref, watch } from "vue";

const props = withDefaults(
  defineProps<{
    modelValue: boolean;
    year: number;
    month: number; // 1-12
    minYear?: number;
    maxYear?: number;
    visibleRows?: number;
    itemHeight?: number;
  }>(),
  {
    minYear: new Date().getFullYear() - 10,
    maxYear: new Date().getFullYear() + 1,
    visibleRows: 5,
    itemHeight: 40,
  }
);

const emit = defineEmits<{
  (e: "update:modelValue", v: boolean): void;
  (e: "update:year", v: number): void;
  (e: "update:month", v: number): void;
  (e: "confirm", payload: { year: number; month: number }): void;
}>();

const localOpen = computed({
  get: () => props.modelValue,
  set: (v: boolean) => emit("update:modelValue", v),
});

const visibleRows = computed(() => props.visibleRows!);
const itemHeight = computed(() => props.itemHeight!);
const columnPadding = computed(
  () => (visibleRows.value * itemHeight.value - itemHeight.value) / 2
);

const yearList = computed<number[]>(() => {
  const list: number[] = [];
  for (let y = props.minYear!; y <= props.maxYear!; y += 1) list.push(y);
  return list;
});

const monthList = computed<number[]>(() =>
  Array.from({ length: 12 }, (_, i) => i + 1)
);

const yearColRef = ref<HTMLElement | null>(null);
const monthColRef = ref<HTMLElement | null>(null);

const snapToIndex = (el: HTMLElement, index: number) => {
  el.scrollTo({ top: index * itemHeight.value, behavior: "auto" });
};

const syncScrollToSelection = () => {
  nextTick(() => {
    if (yearColRef.value) {
      const yIdx = Math.max(0, yearList.value.indexOf(props.year));
      snapToIndex(yearColRef.value, yIdx);
    }
    if (monthColRef.value) {
      const mIdx = Math.max(0, props.month - 1);
      snapToIndex(monthColRef.value, mIdx);
    }
  });
};

watch(localOpen, (open) => {
  if (open) syncScrollToSelection();
});

const onYearScroll = (e: Event) => {
  const el = e.target as HTMLElement;
  const idx = Math.round(el.scrollTop / itemHeight.value);
  const clamped = Math.max(0, Math.min(idx, yearList.value.length - 1));
  emit("update:year", yearList.value[clamped]);
};

const onMonthScroll = (e: Event) => {
  const el = e.target as HTMLElement;
  const idx = Math.round(el.scrollTop / itemHeight.value);
  const clamped = Math.max(0, Math.min(idx, monthList.value.length - 1));
  emit("update:month", monthList.value[clamped]);
};

const close = () => emit("update:modelValue", false);
const confirm = () => emit("confirm", { year: props.year, month: props.month });
</script>

<style scoped>
.picker-container {
  position: relative;
  display: flex;
  gap: 16px;
  align-items: center;
  justify-content: center;
}

.picker-column {
  width: 140px;
  overflow-y: auto;
  scroll-snap-type: y mandatory;
  -ms-overflow-style: none; /* IE/Edge */
  scrollbar-width: none; /* Firefox */
}
.picker-column::-webkit-scrollbar {
  display: none; /* Chrome/Safari */
}

.picker-item {
  height: 40px;
  line-height: 40px;
  text-align: center;
  font-size: 16px;
  scroll-snap-align: center;
}

.picker-indicator {
  position: absolute;
  left: 0;
  right: 0;
  top: 50%;
  transform: translateY(-50%);
  border-top: 1px solid rgba(0, 0, 0, 0.08);
  border-bottom: 1px solid rgba(0, 0, 0, 0.08);
  pointer-events: none;
}
</style>
