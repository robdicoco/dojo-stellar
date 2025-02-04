import { fileURLToPath, URL } from 'node:url'

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueDevTools from 'vite-plugin-vue-devtools'

// https://vite.dev/config/
export default defineConfig({
  plugins: [vue(), vueDevTools()],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url)),
    },
  },
  server: {
    // https: {
    //   key: '/etc/letsencrypt/live/lumen.758206.xyz/privkey.pem',
    //   cert: '/etc/letsencrypt/live/lumen.758206.xyz/fullchain.pem',
    // },
    proxy: {
      '/api': {
        target: 'https://lumen.758206.xyz:7001',
        changeOrigin: true,
        secure: false,
        rewrite: (path) => path.replace(/^\/api/, ''),
      },
    },
  },
})
