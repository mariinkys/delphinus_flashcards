<template>
   <h1 class="text-center mb-3 text-h3">{{ props.headerText }}</h1>
</template>

<script setup lang="ts">
const props = defineProps(['headerText'])
</script>