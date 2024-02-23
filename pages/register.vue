<template>
    <Card class="w-full max-w-5xl mx-auto mt-8">
        <CardHeader>
            <CardTitle>Registrate</CardTitle>
            <CardDescription>Bienvenido, registrate para continuar...</CardDescription>
        </CardHeader>
        <CardContent>
            <form @submit="onSubmit">
                <FormField v-slot="{ componentField }" name="username">
                    <FormItem>
                        <FormLabel>Nombre de Usuario</FormLabel>
                        <FormControl>
                            <Input type="text" placeholder="Usuario" v-bind="componentField" />
                        </FormControl>
                        <FormDescription>
                            Este sera el nombre que usarás para iniciar sesión.
                        </FormDescription>
                        <FormMessage />
                    </FormItem>
                </FormField>
                <br>
                <FormField v-slot="{ componentField }" name="email">
                    <FormItem>
                        <FormLabel>Email</FormLabel>
                        <FormControl>
                            <Input type="text" placeholder="Email" v-bind="componentField" />
                        </FormControl>
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
                <FormField v-slot="{ componentField }" name="repeat_password">
                    <FormItem>
                        <FormLabel>Repite la Contraseña</FormLabel>
                        <FormControl>
                            <Input type="password" placeholder="Repite la Contraseña" v-bind="componentField" />
                        </FormControl>
                        <FormMessage />
                    </FormItem>
                </FormField>
                <br>
                <Button type="submit" :disabled=!passwordsMatch>
                    Registrate
                </Button>
            </form>
        </CardContent>
    </Card>
</template>

<script setup lang="ts">
definePageMeta({ auth: { unauthenticatedOnly: true, navigateAuthenticatedTo: '/home', }, })
import { useToast } from '@/components/ui/toast/use-toast'
const { toast } = useToast()
import axios from 'axios'
import { useForm, useField } from 'vee-validate'
import { toTypedSchema } from '@vee-validate/zod'
import * as z from 'zod'

const formSchema = toTypedSchema(z.object({
    username: z.string().min(2).max(50),
    password: z.string().min(8).max(50),
    email: z.string().email(),
    repeat_password: z.string().min(8).max(50),
}))

const form = useForm({
    validationSchema: formSchema,
})

const { value: password } = useField('password')
const { value: repeatPassword } = useField('repeat_password')

const passwordsMatch = computed(() => {
    if (password.value !== undefined && repeatPassword.value !== undefined) {
        return password.value === repeatPassword.value
    }
    return false
})

const onSubmit = form.handleSubmit(async (values) => {
    if (passwordsMatch) {
        axios.post('/api/register', {
            values
        }).then(async function (res: any) {
            if (res.status == 200) {
                await navigateTo('/')
                toast({
                    title: 'Registered',
                });
            } else {
                toast({
                    title: 'Error',
                });
            }
        }).catch(function (_: any) {
            toast({
                title: 'Error',
            });
        })
    } else {
        toast({
            title: 'Passwords do not match',
        });
    }
})
</script>            