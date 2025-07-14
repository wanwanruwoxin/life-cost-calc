import { createMemoryHistory, createRouter } from 'vue-router'
import type { RouteRecordRaw } from 'vue-router'

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    redirect: '/expense'
  },
  {
    path: '/calculator',
    name: 'calculator',
    component: () => import('../views/Calculator.vue')
  },
  {
    path: '/expense',
    name: 'expense',
    component: () => import('../views/Expense.vue')
  },
  {
    path: '/records',
    name: 'records',
    component: () => import('../views/Records.vue')
  },
  {
    path: '/analysis',
    name: 'analysis',
    component: () => import('../views/Analysis.vue')
  },
  {
    path: '/discover',
    name: 'discover',
    component: () => import('../views/Discover.vue')
  },
  {
    path: '/profile',
    name: 'profile',
    component: () => import('../views/Profile.vue')
  }
]

const router = createRouter({
  history: createMemoryHistory(),
  routes,
})

export default router