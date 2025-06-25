import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import Unocss from 'unocss/vite'
import AutoImport from 'unplugin-auto-import/vite'
import { FileSystemIconLoader } from 'unplugin-icons/loaders'
import IconsResolver from 'unplugin-icons/resolver'
import Icons from 'unplugin-icons/vite'
import { ElementPlusResolver } from 'unplugin-vue-components/resolvers'
import Components from 'unplugin-vue-components/vite'
import { VueRouterAutoImports } from 'unplugin-vue-router'
import type { ComponentResolver } from 'unplugin-vue-components/types'
import VueRouter from 'unplugin-vue-router/vite'
import Layouts from 'vite-plugin-vue-layouts'

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

function IconParkResolver(): ComponentResolver {
  return {
    type: 'component',
    resolve: (name: string) => {
      if (name === 'Icon' || name === 'Iconify')
        return { name: 'Icon', from: '@iconify/vue' }
      return undefined
    },
  }
}

// https://vitejs.dev/config/
export default defineConfig(() => ({
  plugins: [
    vue(),

    VueRouter({
      routesFolder: 'src/pages',
      extensions: ['.vue'],
      dts: 'typed-router.d.ts',
    }),

    // https://github.com/JohnCampionJr/vite-plugin-vue-layouts
    Layouts({
      defaultLayout: 'basic',
      layoutsDirs: 'src/layouts',
    }),

    // https://github.com/antfu/unplugin-auto-import
    AutoImport({
      resolvers: [ElementPlusResolver()],
      imports: ['vue', 'vue-router', '@vueuse/head', '@vueuse/core', VueRouterAutoImports],
      dts: 'src/auto-imports.d.ts',
      dirs: [
        'src/composables/**/*.ts',
        'src/composables/*.ts',
        'src/stores/*.ts',
        'src/utils/*.ts',
      ],
      vueTemplate: true,
    }),
    Components({
      resolvers: [ElementPlusResolver(), IconParkResolver(), IconsResolver({
        prefix: 'icon',
        customCollections: ['local'],
      })],
      extensions: ['vue'],
      include: [/\.vue$/, /\.vue\?vue/],
      directoryAsNamespace: true,
      dts: 'src/components.d.ts',
    }),

    // https://github.com/antfu/unocss
    // 配置见 unocss.config.ts
    Unocss(),

    Icons({
      autoInstall: false,
      customCollections: {
        local: FileSystemIconLoader(
          'src/assets/icons',

          (svg) => {
            // 移除 width 和 height
            return svg.replace(/<svg([^>]+)>/i, (_, p1) => {
              return `<svg${p1.replace(/width="[^"]*"/g, '').replace(/height="[^"]*"/g, '').replace(/fill="[^"]*"/g, 'fill="currentColor"')}>`
            })
          },
        ),
      },
    }),
  ],

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
        protocol: "ws",
        host,
        port: 1421,
      }
      : undefined,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },


}));
