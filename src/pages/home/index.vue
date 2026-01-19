<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

const greetMsg = ref()
async function send() {
  greetMsg.value = await invoke("greet", { name: "测试一下" });

  console.log("greetMsg.value", greetMsg.value)
}

async function send2() {
    greetMsg.value = await invoke("send_message", { value: 120 });

  console.log("greetMsg.value", greetMsg.value)
}

const pm2 = ref()
listen<{
  value: number
}>('watch_message', (event) => {
    console.log(
    `downloading from ${event.payload.value}`
    );
  pm2.value = event.payload.value
})
</script>

<template>
  <div class="min-h-screen flex flex-col">
    <button @click="send">测试</button>

    <button @click="send2" class=" bg-amber mt-10">测试2</button>
    <div>result:{{ pm2 }}</div>
  </div>
</template>

<style scoped>

</style>
