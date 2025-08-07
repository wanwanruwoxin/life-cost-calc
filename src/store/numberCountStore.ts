import { defineStore } from "pinia";
import { Parser } from "expr-eval";

export const useNumberCountStore = defineStore("numberCount", () => {
  const countExpression = ref("0");

  const changeCountExpression = (expression: string) => {
    if (expression === '0' && countExpression.value === '0') {
      return;
    }

    if (expression !== '0' && countExpression.value === '0') {
      countExpression.value = expression;
      return;
    }

    // 校验表达式是否合法，如果合法就把表达式赋值给countExpression
    if (isValidExpression(countExpression.value + expression)) {
      countExpression.value = countExpression.value + expression;
    }
  };

  const isValidExpression = (expression: string) => {
    try {
      // 如果表达式以运算符结尾，添加一个占位符（如 0）
      const normalizedExpression = /[+\-*/]$/.test(expression)
        ? expression + "0"
        : expression;
      Parser.parse(normalizedExpression);
      return true;
    } catch (error) {
      console.warn(error);
      return false;
    }
  };

  const countResult = () => {
    countExpression.value = eval(countExpression.value);
  };

  const popOne = () => {
    countExpression.value = countExpression.value.slice(0, -1);
  };

  const isExpression = computed(() => {
    return !/^\d+$/.test(countExpression.value);
  })

  return {
    countExpression,
    changeCountExpression,
    countResult,
    popOne,
    isExpression,
  };
});
