<template>
    <h1>Index Page</h1>
</template>

<script setup lang="ts">
definePageMeta({ auth: { unauthenticatedOnly: true, navigateAuthenticatedTo: '/home' } })
</script>