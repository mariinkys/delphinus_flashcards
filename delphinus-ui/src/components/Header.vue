<template>
   <v-app-bar :elevation="2" color="primary">
      <div class="d-none d-sm-block">
         <RouterLink to="/" style="text-decoration: none; color: inherit">
            <v-avatar image="/rounded-favicon.png" size="80"></v-avatar>
         </RouterLink>
      </div>
      <v-app-bar-title>
         <RouterLink to="/" style="text-decoration: none; color: inherit">DELPHINUS!</RouterLink>
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