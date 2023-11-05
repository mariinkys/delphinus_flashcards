<template>
  <v-app>
    <v-container>
      <Header />
      <v-main>
        <RouterView />
      </v-main>
    </v-container>
  </v-app>
</template>

<script setup lang="ts">
import Header from "./components/Header.vue";
</script>