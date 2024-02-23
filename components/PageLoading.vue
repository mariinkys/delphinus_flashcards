<script setup lang="ts">
import { ref, watchEffect } from 'vue'
import { Progress } from '@/components/ui/progress'

const progress = ref(13)

watchEffect((cleanupFn) => {
    const timer = setTimeout(() => progress.value = 100, 500)
    cleanupFn(() => clearTimeout(timer))
})
</script>

<template>
    <div class="mt-5">
        <Progress v-model="progress" class="w-3/5 mx-auto" />
    </div>
</template>