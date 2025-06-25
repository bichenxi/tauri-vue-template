import { acceptHMRUpdate, defineStore } from 'pinia'

// 配置状态管理
export const useUserStore = defineStore('user', () => {

  const userInfo = ref({
    name: '张三',
    age: 18,
  })

  const setUserInfo = (info: any) => {
    userInfo.value = info
  }

  return {
    userInfo,
    setUserInfo,
  }
})

if (import.meta.hot)
  import.meta.hot.accept(acceptHMRUpdate(useUserStore, import.meta.hot))
