<template>
  <q-btn
    flex-grow-1
    flex-basis-0
    color="white"
    text-color="black"
    :label="props.label"
    :outline="false"
    square
    @click="handleClick"
  />
</template>

<script setup lang="ts">
import { useNumberCountStore } from "@/store/numberCountStore";

const props = defineProps<{
  label: string;
}>();

const numberCountStore = useNumberCountStore();

const handleClick = () => {
    if (props.label === '=') {
        numberCountStore.countResult();
    } else if (props.label === 'x') {
        numberCountStore.popOne();
    } else {
        numberCountStore.changeCountExpression(props.label);
    }
};
</script>

<style scoped></style>
