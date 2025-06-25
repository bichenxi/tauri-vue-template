# Tauri Vue TypeScript 桌面应用模板

一个现代化的跨平台桌面应用程序开发模板，基于 Tauri + Vue 3 + TypeScript 构建。

## ✨ 特性

- 🚀 **现代技术栈**: Vue 3 + TypeScript + Vite + Tauri
- 📦 **轻量级**: 相比 Electron 更小的包体积和更低的内存占用
- 🎨 **现代 UI**: UnoCSS 原子化 CSS + Element Plus 组件库
- 🔧 **开发体验**: 热重载、自动导入、文件路由系统
- 🛡️ **类型安全**: 全面的 TypeScript 支持
- 🌐 **跨平台**: 支持 Windows、macOS、Linux
- ⚡ **高性能**: Rust 后端提供系统级API调用能力

## 🛠️ 技术栈

### 前端
- **框架**: Vue 3 (Composition API)
- **语言**: TypeScript
- **构建工具**: Vite
- **状态管理**: Pinia
- **路由**: Vue Router (文件路由)
- **UI 框架**: Element Plus
- **样式**: UnoCSS
- **图标**: Iconify + 自定义图标

### 后端
- **运行时**: Tauri (Rust)
- **API**: Tauri Commands

### 开发工具
- **自动导入**: unplugin-auto-import
- **组件自动导入**: unplugin-vue-components
- **文件路由**: unplugin-vue-router
- **布局系统**: vite-plugin-vue-layouts
- **图标处理**: unplugin-icons

## 📂 项目结构

```
tauri-vue-template/
├── src/                    # 前端源码
│   ├── assets/            # 静态资源
│   ├── components/        # Vue 组件
│   ├── composables/       # 组合式函数
│   ├── layouts/           # 布局组件
│   ├── pages/             # 页面组件 (自动路由)
│   ├── stores/            # Pinia 状态管理
│   ├── types/             # TypeScript 类型定义
│   ├── utils/             # 工具函数
│   ├── App.vue           # 根组件
│   └── main.ts           # 应用入口
├── src-tauri/             # Tauri 后端
│   ├── src/              # Rust 源码
│   ├── icons/            # 应用图标
│   ├── Cargo.toml        # Rust 依赖配置
│   └── tauri.conf.json   # Tauri 配置
├── public/                # 公共静态资源
├── package.json          # Node.js 依赖配置
├── vite.config.ts        # Vite 配置
├── unocss.config.ts      # UnoCSS 配置
└── tsconfig.json         # TypeScript 配置
```

## 🚀 快速开始

### 环境要求

- Node.js >= 18
- pnpm (推荐) 或 npm/yarn
- Rust (最新稳定版)
- 系统依赖 (根据目标平台)

### 安装依赖

```bash
# 克隆项目
git clone <repository-url>
cd tauri-vue-template

# 安装前端依赖
pnpm install

# 安装 Tauri CLI (如果未安装)
pnpm add -g @tauri-apps/cli
```

### 开发模式

```bash
# 启动开发服务器
pnpm tauri dev
```

这将同时启动前端开发服务器和 Tauri 应用。

### 构建应用

```bash
# 构建生产版本
pnpm tauri build
```

构建产物将输出到 `src-tauri/target/release/bundle/` 目录。

## 📝 开发指南

### 添加新页面

在 `src/pages/` 目录下创建 `.vue` 文件，路由将自动生成：

```bash
src/pages/
├── index.vue          # 路由: /
├── about.vue          # 路由: /about
└── user/
    ├── index.vue      # 路由: /user
    └── profile.vue    # 路由: /user/profile
```

### 状态管理

使用 Pinia 进行状态管理，在 `src/stores/` 目录下创建 store：

```typescript
// src/stores/user.ts
export const useUserStore = defineStore('user', () => {
  const name = ref('')
  
  const setName = (newName: string) => {
    name.value = newName
  }
  
  return { name, setName }
})
```

### 调用 Rust 后端

在前端调用 Tauri 命令：

```typescript
import { invoke } from '@tauri-apps/api/core'

// 调用后端命令
const result = await invoke('greet', { name: 'World' })
```

### 样式开发

使用 UnoCSS 原子化 CSS：

```vue
<template>
  <div class="flex items-center justify-center p-4 bg-blue-500 text-white rounded-lg">
    Hello World
  </div>
</template>
```

### 组件使用

Element Plus 组件自动导入，无需手动导入：

```vue
<template>
  <el-button type="primary" @click="handleClick">
    点击我
  </el-button>
</template>
```

## 🔧 配置说明

### Tauri 配置

主要配置文件：`src-tauri/tauri.conf.json`

- 应用信息配置
- 窗口设置
- 权限配置
- 构建选项

### Vite 配置

主要配置文件：`vite.config.ts`

- 插件配置
- 自动导入设置
- 开发服务器配置

### UnoCSS 配置

主要配置文件：`unocss.config.ts`

- 预设配置
- 自定义规则
- 主题变量

## 📦 打包发布

### 本地构建

```bash
# 构建应用
pnpm tauri build

# 构建特定平台 (可选)
pnpm tauri build --target x86_64-pc-windows-msvc
```

### 应用图标

将应用图标放置在 `src-tauri/icons/` 目录下：

- `32x32.png`
- `128x128.png`
- `128x128@2x.png`
- `icon.icns` (macOS)
- `icon.ico` (Windows)

## 🤝 贡献指南

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

## 📄 许可证

本项目基于 MIT 许可证开源。详情请参阅 [LICENSE](LICENSE) 文件。

## 🔗 相关链接

- [Tauri 官方文档](https://tauri.app/)
- [Vue 3 官方文档](https://vuejs.org/)
- [UnoCSS 官方文档](https://unocss.dev/)
- [Element Plus 官方文档](https://element-plus.org/)
- [Pinia 官方文档](https://pinia.vuejs.org/)

## 💡 问题反馈

如遇问题或有改进建议，请通过以下方式反馈：

- 提交 [Issue](../../issues)
- 发起 [Discussion](../../discussions)

---

**祝你使用愉快！** 🎉
