import { acceptHMRUpdate, defineStore } from 'pinia'
import { ofetch } from 'ofetch'

export const useTaskStore = defineStore('task', () => {

  const taskList = ref([])
  // 获取搜索任务列表
  const fetchTaskList = async (query: string) => {
    const res = await ofetch(`${import.meta.env.VITE_API_URL}/relay/web-search`, {
      method: 'GET',
      params: {
        query: query,
        engine: 'aliyun',
        time_range: 'NoLimit',
        source: 6
      }
    })
    if (res.code === 0) {
      taskList.value = res.data.search_result
    }
  }

  return {
    taskList,
    fetchTaskList,
  }
})

if (import.meta.hot)
  import.meta.hot.accept(acceptHMRUpdate(useTaskStore, import.meta.hot))