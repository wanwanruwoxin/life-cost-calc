<script setup lang="ts">
import { ref } from "vue";
import { useQuasar } from "quasar";

const $q = useQuasar();

// 用户信息
const userInfo = ref({
  name: "用户",
  avatar: "",
  joinDate: "2024-01-01",
  totalRecords: 15,
  totalDays: 20,
});

// 设置选项
const settingGroups = ref([
  {
    title: "账户设置",
    items: [
      {
        icon: "person",
        title: "个人信息",
        subtitle: "修改头像、昵称等",
        action: "profile",
      },
      {
        icon: "security",
        title: "隐私设置",
        subtitle: "数据安全与隐私保护",
        action: "privacy",
      },
    ],
  },
  {
    title: "应用设置",
    items: [
      {
        icon: "palette",
        title: "主题设置",
        subtitle: "切换深色/浅色主题",
        action: "theme",
      },
      {
        icon: "language",
        title: "语言设置",
        subtitle: "选择应用语言",
        action: "language",
      },
      {
        icon: "notifications",
        title: "通知设置",
        subtitle: "管理推送通知",
        action: "notifications",
      },
    ],
  },
  {
    title: "数据管理",
    items: [
      {
        icon: "backup",
        title: "数据备份",
        subtitle: "备份您的记账数据",
        action: "backup",
      },
      {
        icon: "restore",
        title: "数据恢复",
        subtitle: "从备份恢复数据",
        action: "restore",
      },
      {
        icon: "delete_forever",
        title: "清空数据",
        subtitle: "删除所有记账记录",
        action: "clear",
        danger: true,
      },
    ],
  },
  {
    title: "关于",
    items: [
      {
        icon: "help",
        title: "使用帮助",
        subtitle: "查看使用说明",
        action: "help",
      },
      {
        icon: "info",
        title: "关于应用",
        subtitle: "版本信息与开发者",
        action: "about",
      },
    ],
  },
]);

const handleAction = (action: string) => {
  switch (action) {
    case "profile":
      $q.notify({ message: "个人信息设置开发中...", color: "info" });
      break;
    case "privacy":
      $q.notify({ message: "隐私设置开发中...", color: "info" });
      break;
    case "theme":
      $q.notify({ message: "主题设置开发中...", color: "info" });
      break;
    case "language":
      $q.notify({ message: "语言设置开发中...", color: "info" });
      break;
    case "notifications":
      $q.notify({ message: "通知设置开发中...", color: "info" });
      break;
    case "backup":
      $q.notify({ message: "数据备份功能开发中...", color: "info" });
      break;
    case "restore":
      $q.notify({ message: "数据恢复功能开发中...", color: "info" });
      break;
    case "clear":
      $q.dialog({
        title: "确认清空",
        message: "此操作将删除所有记账数据，且无法恢复。确定要继续吗？",
        cancel: true,
        persistent: true,
      }).onOk(() => {
        $q.notify({ message: "数据清空功能开发中...", color: "warning" });
      });
      break;
    case "help":
      $q.notify({ message: "帮助文档开发中...", color: "info" });
      break;
    case "about":
      $q.dialog({
        title: "关于生活成本计算器",
        message: "版本：1.0.0\n开发者：个人项目\n技术栈：Vue3 + Quasar + Tauri",
        ok: "确定",
      });
      break;
    default:
      $q.notify({ message: "功能开发中...", color: "info" });
  }
};
</script>

<template>
  <div class="w-full max-w-md mx-auto">
    <!-- 用户信息卡片 -->
    <div class="bg-gradient-to-br from-indigo-500 to-purple-600 p-6 text-white">
      <div class="flex items-center mb-4">
        <QAvatar size="60px" class="mr-4">
          <QIcon name="person" size="2rem" />
        </QAvatar>
        <div>
          <h2 class="text-xl font-bold">{{ userInfo.name }}</h2>
          <p class="text-indigo-100 text-sm">加入于 {{ userInfo.joinDate }}</p>
        </div>
      </div>

      <div class="grid grid-cols-2 gap-4">
        <div class="text-center">
          <div class="text-2xl font-bold">{{ userInfo.totalRecords }}</div>
          <div class="text-indigo-100 text-sm">记录条数</div>
        </div>
        <div class="text-center">
          <div class="text-2xl font-bold">{{ userInfo.totalDays }}</div>
          <div class="text-indigo-100 text-sm">使用天数</div>
        </div>
      </div>
    </div>

    <!-- 设置列表 -->
    <div class="p-4 space-y-6">
      <div v-for="group in settingGroups" :key="group.title">
        <h3 class="text-sm font-medium text-gray-500 mb-3 px-2">
          {{ group.title }}
        </h3>

        <QCard class="rounded-3 shadow-sm">
          <QList>
            <QItem
              v-for="(item, index) in group.items"
              :key="item.action"
              clickable
              v-ripple
              @click="handleAction(item.action)"
              class="border-b border-gray-100 last:border-b-0"
            >
              <QItemSection avatar>
                <QIcon
                  :name="item.icon"
                  :color="item.danger ? 'negative' : 'primary'"
                  size="md"
                />
              </QItemSection>

              <QItemSection>
                <QItemLabel
                  class="font-medium"
                  :class="item.danger ? 'text-red-600' : 'text-gray-800'"
                >
                  {{ item.title }}
                </QItemLabel>
                <QItemLabel caption class="text-gray-500 text-xs mt-1">
                  {{ item.subtitle }}
                </QItemLabel>
              </QItemSection>

              <QItemSection side>
                <QIcon name="chevron_right" color="grey-4" />
              </QItemSection>
            </QItem>
          </QList>
        </QCard>
      </div>

      <!-- 版本信息 -->
      <div class="text-center pt-4">
        <p class="text-xs text-gray-400">生活成本计算器 v1.0.0</p>
        <p class="text-xs text-gray-400 mt-1">© 2024 个人项目</p>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* 列表项悬停效果 */
.q-item:hover {
  background-color: #f9fafb;
}
</style>
