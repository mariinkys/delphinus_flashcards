import { defineStore } from 'pinia'

export const useFoundCharsStore = defineStore('foundChars', {
  state: () => ({ characters: null }),
  getters: {
    getData: (state) => state.characters,
  },
  actions: {
    updateData(data: any) {
      this.characters = data
    },
  },
})
