import { createApp } from "vue";
import App from "./App.vue";
import { setupLayouts } from 'virtual:generated-layouts'
import { routes } from 'vue-router/auto-routes'
import { createRouter, createWebHistory } from 'vue-router'


const routeList = setupLayouts(routes)

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: routeList,
})
// 不使用ssg
const app = createApp(App)
app.use(router)
app.mount("#app");
