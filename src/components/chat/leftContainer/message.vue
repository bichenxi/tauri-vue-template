<script lang="ts" setup>
import MarkdownIt from 'markdown-it'

const props = defineProps<{
  value: string
}>()

const md = new MarkdownIt({
  html: true,
  breaks: true,
  linkify: true,
})

const rendered = shallowRef<string>('')

watch(
  () => props.value,
  (newValue) => {
    rendered.value = md.render(newValue)
  },
  { immediate: true },
)
// 监听 props.value 的变化，重新渲染 Markdown
</script>

<template>
  <div v-html="rendered" />
</template>
