import { fileURLToPath, URL } from 'node:url'

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import { resolve } from 'path'
// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    vue(),
  ],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url))
    }
  },
  base:"",
  build:{
    rollupOptions: {
      input: {
        front: resolve(__dirname, 'src/front.js'),
        background: resolve(__dirname, 'src/background.js'),
        sidepanel: resolve(__dirname, 'sidepanel.html'),
      },
      output:{
        dynamicImportInCjs:false,
        entryFileNames: '[name].js',
      }
    },
  }
})
