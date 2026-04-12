import { createApp } from 'vue'
import { createPinia } from 'pinia'
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import * as ElementPlusIconsVue from '@element-plus/icons-vue'

// 导入自定义主题样式
import './styles/theme.css'

import App from './App.vue'
import router from './router'

const app = createApp(App)

// Register all Element Plus icons
for (const [key, component] of Object.entries(ElementPlusIconsVue)) {
  app.component(key, component)
}

// SES 主题色配置
const sesTheme = {
  primary: 'rgba(46, 198, 214, 1)',
  primaryLight: 'rgba(46, 198, 214, 0.8)',
  primaryLighter: 'rgba(46, 198, 214, 0.1)',
}

// 全局挂载主题配置
app.config.globalProperties.$sesTheme = sesTheme

app.use(createPinia())
app.use(router)
app.use(ElementPlus, {
  // Element Plus 主题配置
  zIndex: 3000,
})

app.mount('#app')
