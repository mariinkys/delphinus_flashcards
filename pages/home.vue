<template>
    <div class="flex gap-1 justify-between items-center">
        <PageTitle>Home</PageTitle>
        <div class="flex gap-1 items-center">
            <StudysetCreateStudySet @refresh-study-sets="() => refreshModel()" />
        </div>
    </div>

    <div v-if="!loading">
        <div v-if="studySets.length > 0" class="flex gap-2 mt-3">
            <div v-for="studySet in studySets" class="items-center flex border py-2 min-w-32 justify-around">
                <p @click="() => navigateTo(`/studyset/${studySet.id}`)">{{ studySet.name }}</p>
                <DropdownMenu>
                    <DropdownMenuTrigger>···</DropdownMenuTrigger>
                    <DropdownMenuContent>
                        <DropdownMenuLabel>{{ studySet.name }} - StudySet</DropdownMenuLabel>
                        <DropdownMenuSeparator />
                        <DropdownMenuItem @Click="deleteStudySet(studySet.id)">Delete</DropdownMenuItem>
                    </DropdownMenuContent>
                </DropdownMenu>
            </div>
        </div>
        <div class="text-center" v-else>
            <p class="mt-5">No study sets...</p>
        </div>
    </div>
    <div v-else>
        <PageLoading />
    </div>
</template>

<script lang="ts">
import axios from 'axios'
import { useToast } from '@/components/ui/toast/use-toast'
const { toast } = useToast()
import type { StudySetModel } from '@/server/models/studyset'

export default {
    data() {
        return {
            studySets: ref<StudySetModel[]>([]),
            loading: true,
        }
    },
    async mounted() {
        await this.refreshModel();
    },
    methods: {
        async refreshModel() {
            this.loading = true;
            await axios.get(`/api/studysets`)
                .then(res => {
                    if (res.status == 200) {
                        this.studySets = res.data
                        this.loading = false
                    } else {
                        toast({
                            title: 'Error',
                        });
                    }
                })
        },
        async deleteStudySet(id: number | null) {
            if (id) {
                const data = {
                    id: id,
                }
                this.loading = true;
                await axios.post(`/api/studysets/delete`, {
                    data
                }).then(async res => {
                    if (res.status == 200) {
                        await this.refreshModel()
                    } else {
                        this.loading = false;
                        toast({
                            title: 'Error',
                        });
                    }
                }).catch(_ => {
                    this.loading = false;
                    toast({
                        title: 'Error',
                    });
                })
            }
        }
    }
}
</script>