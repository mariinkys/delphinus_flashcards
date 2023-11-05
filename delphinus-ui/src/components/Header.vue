<template>
   <v-app-bar :elevation="2" color="primary">
      <v-app-bar-title>
         <RouterLink to="/" style="text-decoration: none; color: inherit">Delphinus!</RouterLink>
      </v-app-bar-title>
      <v-spacer />
      <v-app-bar-nav-icon to="/faq" icon="mdi-frequently-asked-questions"></v-app-bar-nav-icon>
      <v-btn @click="toggleTheme" icon="mdi-theme-light-dark"></v-btn>
   </v-app-bar>
</template>

<script setup lang="ts">
import { RouterLink } from 'vue-router';
import { useTheme } from 'vuetify'

const theme = useTheme()

function toggleTheme() {
   theme.global.name.value = theme.global.current.value.dark ? 'light' : 'dark'
}
</script>