// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  devtools: { enabled: true },
  modules: ["@sidebase/nuxt-auth", "@nuxtjs/tailwindcss", "shadcn-nuxt"],
  auth: {
    provider: { type: 'authjs' },
    globalAppMiddleware: true,
    origin: 'http://localhost:3000'
  },
  shadcn: {
    prefix: '',
    componentDir: './components/ui'
  }
})