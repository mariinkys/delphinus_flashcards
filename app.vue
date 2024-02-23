<template>
  <div id="app" v-cloak>

    <Head>
      <Title>Delphinus Flashcards</Title>
    </Head>

    <NuxtLayout>
      <Navbar :logged-in="loggedIn" />
      <div class="p-4">
        <NuxtPage />
      </div>
    </NuxtLayout>

    <Toaster />
  </div>
</template>

<script setup lang="ts">
const { status } = useAuth()
const loggedIn = computed(() => status.value === 'authenticated')

useSeoMeta({
  description: 'Delphinus Flashcards',
  ogTitle: 'Delphinus',
  ogDescription: 'Delphinus Flashcards',
  ogImage: '/favicon.ico',
  // ogUrl: '[og:url]',
  // twitterTitle: '[twitter:title]',
  // twitterDescription: '[twitter:description]',
  // twitterImage: '[twitter:image]',
  // twitterCard: 'summary'
})

useHead({
  htmlAttrs: {
    lang: 'en'
  },
  link: [
    {
      rel: 'icon',
      type: 'image/png',
      href: '/favicon.ico'
    }
  ]
})
</script>