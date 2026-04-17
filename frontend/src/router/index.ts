import { createRouter, createWebHistory } from 'vue-router'
import Layout from '../views/Layout.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      component: Layout,
      redirect: '/dashboard',
      children: [
        {
          path: '',
          name: 'Home',
          component: () => import('../views/Home.vue'),
        },
        {
          path: 'dashboard',
          name: 'Dashboard',
          component: () => import('../views/Dashboard.vue'),
          meta: { title: '系统概览', icon: 'Odometer' }
        },
        {
          path: 'flows',
          name: 'Flows',
          component: () => import('../views/Flows.vue'),
          meta: { title: '工作流管理', icon: 'Connection' }
        },
        {
          path: 'flows/:id/editor',
          name: 'FlowEditor',
          component: () => import('../views/FlowEditor.vue'),
          meta: { title: '工作流编辑器', hideInMenu: true }
        },
        {
          path: 'nodes',
          name: 'Nodes',
          component: () => import('../views/Nodes.vue'),
          meta: { title: '节点管理', icon: 'Grid' }
        },
        {
          path: 'devices',
          name: 'Devices',
          component: () => import('../views/Devices.vue'),
          meta: { title: '设备管理', icon: 'Cpu' }
        },
        {
          path: 'apps',
          name: 'Apps',
          component: () => import('../views/Apps.vue'),
          meta: { title: '应用管理', icon: 'Collection' }
        },
        {
          path: 'integration',
          name: 'Integration',
          component: () => import('../views/Integration.vue'),
          meta: { title: '外部系统对接', icon: 'Link' }
        },
      ],
    },
  ],
})

export default router
