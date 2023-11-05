import { defineStore } from 'pinia'

export const useResultStore = defineStore('result', {
   state: () => ({ result: '' }),
   getters: {
      getData: (state) => state.result,
   },
   actions: {
      updateData(data: string) {
         this.result = data
      },
   },
})
