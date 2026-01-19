<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";

// const greetMsg = ref("");
// const name = ref("");

// async function greet() {
//   // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
//   greetMsg.value = await invoke("greet", { name: name.value });
// }

interface ServerInfo {
  running: boolean
  port: number
  localUrl: string
  networkUrl: string
}

const serverInfo = ref<ServerInfo | null>(null)
const loading = ref(false)
const error = ref('')

// 启动服务器
async function startServer() {
  console.log("VITE_BASE_PORT", import.meta.env.VITE_BASE_PORT)
  try {
    serverInfo.value = await invoke<ServerInfo>('start_h5_server', { port: +import.meta.env.VITE_BASE_PORT })
  } catch (e: any) {
    error.value = e.toString()
  } finally {
    loading.value = false
  }
}

onMounted(() => {
  startServer()
})
</script>

<template>
  <RouterView />
</template>

<style scoped>

</style>