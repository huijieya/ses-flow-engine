import { createRouter, createWebHistory } from 'vue-router'
import Layout from '../views/Layout.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      component: Layout,
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
        },
        {
          path: 'flows',
          name: 'Flows',
          component: () => import('../views/Flows.vue'),
        },
        {
          path: 'flows/:id/editor',
          name: 'FlowEditor',
          component: () => import('../views/FlowEditor.vue'),
        },
        {
          path: 'nodes',
          name: 'Nodes',
          component: () => import('../views/Nodes.vue'),
        },
        {
          path: 'devices',
          name: 'Devices',
          component: () => import('../views/Devices.vue'),
        },
        {
          path: 'apps',
          name: 'Apps',
          component: () => import('../views/Apps.vue'),
        },
        {
          path: 'integration',
          name: 'Integration',
          component: () => import('../views/Integration.vue'),
        },
      ],
    },
  ],
})

export default router
