<script setup lang="ts">
import axios from 'axios'

let value = ref(0)
function sendMessage() {
  axios.post("/api/v1/set_device", {
    pm2: value.value
  })
}

</script>

<template>
  <div class="min-h-screen flex flex-col items-center justify-center bg-gradient-to-br from-slate-900 via-purple-900 to-slate-900 p-8">
    配置页面
    <input type="number" v-model="value" >
    <button @click="sendMessage">发送数据</button>
  </div>
</template>

<style scoped>
</style>
  