<template>
    <nav class="flex h-16 items-center px-4 justify-between">
        <NuxtLink :to="props.loggedIn ? '/home' : '/'" style="color: inherit; text-decoration: none;">
            Inicio
        </NuxtLink>
        <div v-if="props.loggedIn">
            <Button class="mx-1" @click="signOut({ callbackUrl: '/' })">Salir</Button>
        </div>
        <div v-else class="flex gap-2 items-center">
            <NuxtLink to="/login" style="color: inherit; text-decoration: none;">
                <Button class="mx-1">Inicia Sesión</Button>
            </NuxtLink>
            <NuxtLink to="/register" style="color: inherit; text-decoration: none;">
                <Button class="mx-1">Register</Button>
            </NuxtLink>
        </div>
    </nav>
</template>

<script setup lang="ts">
const props = defineProps(['loggedIn'])
const { signOut } = useAuth()

</script>