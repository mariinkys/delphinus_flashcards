<template>
    <Card class="w-full max-w-5xl mx-auto mt-8">
        <CardHeader>
            <CardTitle>Inicio de Sesión</CardTitle>
            <CardDescription>Bienvenido, inicia sesión para continuar...</CardDescription>
        </CardHeader>
        <CardContent>
            <form @submit="onSubmit">
                <FormField v-slot="{ componentField }" name="username">
                    <FormItem>
                        <FormLabel>Nombre de Usuario</FormLabel>
                        <FormControl>
                            <Input type="text" placeholder="Usuario" v-bind="componentField" />
                        </FormControl>
                        <!-- <FormDescription>
                                This is your public display name.
                            </FormDescription> -->
                        <FormMessage />
                    </FormItem>
                </FormField>
                <br>
                <FormField v-slot="{ componentField }" name="password">
                    <FormItem>
                        <FormLabel>Contraseña</FormLabel>
                        <FormControl>
                            <Input type="password" placeholder="Contraseña" v-bind="componentField" />
                        </FormControl>
                        <FormMessage />
                    </FormItem>
                </FormField>
                <br>
                <Button type="submit">
                    Inicia Sesión
                </Button>
            </form>
        </CardContent>
    </Card>
</template>

<script setup lang="ts">
definePageMeta({ auth: { unauthenticatedOnly: true, navigateAuthenticatedTo: '/home', }, })
const { signIn } = useAuth()
import { useForm } from 'vee-validate'
import { toTypedSchema } from '@vee-validate/zod'
import * as z from 'zod'

const formSchema = toTypedSchema(z.object({
    username: z.string().min(2).max(50),
    password: z.string().min(8).max(50),
}))

const form = useForm({
    validationSchema: formSchema,
})

const onSubmit = form.handleSubmit(async (values) => {
    const user = values.username;
    const password = values.password;
    await signIn('credentials', { user, password })
})
</script>            