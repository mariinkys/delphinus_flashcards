<template>
    <Dialog v-model:open="open">
        <DialogTrigger as-child>
            <Button>
                New Study Set
            </Button>
        </DialogTrigger>
        <DialogContent class="sm:max-w-[425px]">
            <DialogHeader>
                <DialogTitle>New Study Set</DialogTitle>
                <DialogDescription>
                    Click save when you're done.
                </DialogDescription>
            </DialogHeader>
            <form @submit="onSubmit">
                <FormField v-slot="{ componentField }" name="name" class="grid gap-4 py-4">
                    <FormItem class="grid grid-cols-4 items-center gap-4">
                        <FormLabel class="text-right mt-1">Name</FormLabel>
                        <FormControl>
                            <Input type="text" v-bind="componentField" class="col-span-3" />
                        </FormControl>
                    </FormItem>
                </FormField>
                <DialogFooter class="mt-3">
                    <Button type="submit">
                        Save changes
                    </Button>
                </DialogFooter>
            </form>
        </DialogContent>
    </Dialog>
</template>

<script setup lang="ts">
import { initDefaultStudySet } from '~/server/models/studyset'
import { useForm } from 'vee-validate'
import { toTypedSchema } from '@vee-validate/zod'
import { useToast } from '@/components/ui/toast/use-toast'
const { toast } = useToast()
import axios from 'axios'
import * as z from 'zod'

const studySet = initDefaultStudySet()
const emit = defineEmits(['refreshStudySets'])
let open = ref(false)

const formSchema = toTypedSchema(z.object({
    name: z.string().min(2).max(50),
}))

const form = useForm({
    validationSchema: formSchema,
})

const onSubmit = form.handleSubmit(async (values) => {
    studySet.name = values.name;
    axios.post('/api/studysets', {
        studySet
    }).then(async function (res: any) {
        if (res.status == 200) {
            emit('refreshStudySets');
            open.value = false;
            toast({
                title: 'Created',
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
})
</script>