import { createApp } from "vue";
import App from "./App.vue";
import { setupLayouts } from 'virtual:generated-layouts'
import { routes } from 'vue-router/auto-routes'
import { createRouter, createWebHistory } from 'vue-router'
import 'uno.css'
import './styles/main.css'
import { createPinia } from 'pinia'


const routeList = setupLayouts(routes)

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: routeList,
})

// 创建 Pinia 实例
const pinia = createPinia()

// 不使用ssg
const app = createApp(App)
app.use(router)
// 使用 Pinia
app.use(pinia)
app.mount("#app");
