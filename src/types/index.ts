// 频率类型枚举
export type FrequencyType = "daily" | "weekly" | "monthly" | "yearly";

// 频率配置接口
export interface FrequencyConfig {
  label: string;
  icon: string;
  monthlyMultiplier: number;
  yearlyMultiplier: number;
}

// 计算结果接口
export interface PriceCalcResult {
  monthly: number;
  yearly: number;
}

// 费用记录接口
export interface ExpenseRecord {
  id: string;
  price: number;
  description: string;
  frequency: FrequencyType;
  createdAt: Date;
  updatedAt?: Date;
  monthlyPrice: number;
  yearlyPrice: number;
}

// 频率配置常量
export const FREQUENCY_CONFIGS: Record<FrequencyType, FrequencyConfig> = {
  daily: {
    label: "每天",
    icon: "today",
    monthlyMultiplier: 30,
    yearlyMultiplier: 365,
  },
  weekly: {
    label: "每周",
    icon: "date_range",
    monthlyMultiplier: 4,
    yearlyMultiplier: 52,
  },
  monthly: {
    label: "每月",
    icon: "calendar_month",
    monthlyMultiplier: 1,
    yearlyMultiplier: 12,
  },
  yearly: {
    label: "每年",
    icon: "calendar_today",
    monthlyMultiplier: 1 / 12,
    yearlyMultiplier: 1,
  },
};

// 获取所有频率选项
export const FREQUENCY_OPTIONS = Object.keys(
  FREQUENCY_CONFIGS
) as FrequencyType[];
