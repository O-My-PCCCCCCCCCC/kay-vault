import { createApp } from 'vue'
import { createPinia } from 'pinia'
import NaiveUI from 'naive-ui'
import App from './App.vue'
import router from './router'
import './styles/theme.css'

// 在 Vue 挂载前就设置主题，避免闪烁
const saved = localStorage.getItem('kayTheme')
if (saved) document.documentElement.setAttribute('data-theme', saved)

const app = createApp(App)
app.use(createPinia())
app.use(router)
app.use(NaiveUI)
app.mount('#app')
