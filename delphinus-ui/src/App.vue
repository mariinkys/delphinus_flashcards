<template>
  <v-app>
    <v-container class="app-container">
      <Header />
      <v-main>
        <RouterView />
      </v-main>
    </v-container>
    <Footer />
  </v-app>
</template>

<script setup lang="ts">
import Header from "./components/Header.vue";
import Footer from "./components/Footer.vue";
</script>

<style>
.app-container {
  display: flex;
  flex-direction: column;
  min-height: 100vh;
}

.v-main {
  flex: 1;
}

footer {
  width: 100%;
  position: relative;
  bottom: 0;
}
</style>