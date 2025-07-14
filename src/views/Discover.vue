<script setup lang="ts">
import { ref } from "vue";

// 发现页面功能列表
const features = ref([
  {
    id: 1,
    title: "预算管理",
    description: "设置月度预算，追踪支出进度",
    icon: "savings",
    color: "blue",
    comingSoon: true,
  },
  {
    id: 2,
    title: "账单提醒",
    description: "设置定期账单提醒，不再错过缴费",
    icon: "notifications",
    color: "orange",
    comingSoon: true,
  },
  {
    id: 3,
    title: "理财建议",
    description: "基于支出分析的个性化理财建议",
    icon: "lightbulb",
    color: "green",
    comingSoon: true,
  },
  {
    id: 4,
    title: "数据导出",
    description: "导出支出数据为Excel或PDF格式",
    icon: "download",
    color: "purple",
    comingSoon: false,
  },
  {
    id: 5,
    title: "分类管理",
    description: "自定义支出分类，更精准记录",
    icon: "category",
    color: "teal",
    comingSoon: true,
  },
  {
    id: 6,
    title: "多账户",
    description: "管理多个账户的收支情况",
    icon: "account_balance",
    color: "indigo",
    comingSoon: true,
  },
]);

// 最新动态
const updates = ref([
  {
    id: 1,
    title: "新增数据分析功能",
    description: "现在可以查看详细的支出分析和统计图表",
    date: "2024-01-20",
    type: "feature",
  },
  {
    id: 2,
    title: "界面优化更新",
    description: "优化了移动端界面体验，操作更加流畅",
    date: "2024-01-15",
    type: "improvement",
  },
  {
    id: 3,
    title: "修复已知问题",
    description: "修复了计算器页面的一些显示问题",
    date: "2024-01-10",
    type: "bugfix",
  },
]);

const getUpdateIcon = (type: string) => {
  const icons: Record<string, string> = {
    feature: "new_releases",
    improvement: "upgrade",
    bugfix: "bug_report",
  };
  return icons[type] || "info";
};

const getUpdateColor = (type: string) => {
  const colors: Record<string, string> = {
    feature: "green",
    improvement: "blue",
    bugfix: "orange",
  };
  return colors[type] || "grey";
};
</script>

<template>
  <div class="w-full max-w-md mx-auto p-4">
    <!-- 页面标题 -->
    <div class="mb-6">
      <h1 class="text-2xl font-bold text-gray-800 mb-2">发现</h1>
      <p class="text-gray-600 text-sm">探索更多实用功能</p>
    </div>

    <!-- 功能卡片网格 -->
    <div class="grid grid-cols-2 gap-4 mb-8">
      <QCard
        v-for="feature in features"
        :key="feature.id"
        class="rounded-3 shadow-md hover:shadow-lg transition-all duration-300 cursor-pointer transform hover:-translate-y-1"
        @click="feature.comingSoon ? null : null"
      >
        <QCardSection class="p-4 text-center relative">
          <div v-if="feature.comingSoon" class="absolute top-2 right-2">
            <QBadge color="orange" label="敬请期待" class="text-xs" />
          </div>

          <div class="mb-3">
            <QIcon
              :name="feature.icon"
              size="2rem"
              :class="`text-${feature.color}-500`"
            />
          </div>

          <h3 class="text-sm font-semibold text-gray-800 mb-2">
            {{ feature.title }}
          </h3>
          <p class="text-xs text-gray-600 leading-relaxed">
            {{ feature.description }}
          </p>
        </QCardSection>
      </QCard>
    </div>

    <!-- 最新动态 -->
    <QCard class="rounded-3 shadow-md mb-6">
      <QCardSection
        class="bg-gradient-to-br from-blue-50 to-indigo-50 rounded-t-3 border-b border-indigo-100"
      >
        <div class="text-lg font-medium flex items-center">
          <QIcon name="timeline" class="mr-2" />
          最新动态
        </div>
      </QCardSection>

      <QCardSection class="p-0">
        <QList>
          <QItem
            v-for="update in updates"
            :key="update.id"
            class="border-b border-gray-100 last:border-b-0"
          >
            <QItemSection avatar>
              <QIcon
                :name="getUpdateIcon(update.type)"
                :color="getUpdateColor(update.type)"
                size="md"
              />
            </QItemSection>

            <QItemSection>
              <QItemLabel class="font-medium text-gray-800">{{
                update.title
              }}</QItemLabel>
              <QItemLabel caption class="text-gray-600 text-xs mt-1">{{
                update.description
              }}</QItemLabel>
              <QItemLabel caption class="text-gray-400 text-xs mt-1">{{
                update.date
              }}</QItemLabel>
            </QItemSection>
          </QItem>
        </QList>
      </QCardSection>
    </QCard>

    <!-- 反馈建议 -->
    <QCard class="rounded-3 shadow-md">
      <QCardSection class="text-center p-6">
        <QIcon name="feedback" size="2rem" class="text-blue-500 mb-3" />
        <h3 class="text-lg font-semibold text-gray-800 mb-2">意见反馈</h3>
        <p class="text-sm text-gray-600 mb-4">您的建议是我们改进的动力</p>

        <QBtn
          color="primary"
          rounded
          unelevated
          class="px-6"
          @click="$q.notify({ message: '反馈功能开发中...', color: 'info' })"
        >
          <QIcon name="send" class="mr-2" />
          提交反馈
        </QBtn>
      </QCardSection>
    </QCard>
  </div>
</template>

<style scoped>
/* 卡片悬停效果 */
.q-card:hover {
  transform: translateY(-2px);
}
</style>
