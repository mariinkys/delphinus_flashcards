<template>
   <h1>Results Found!</h1>
   <div v-if="!wantsToCompose">
      <p>Do you want to revise the flashcards?</p>
      <v-btn @click="wantsToCompose = true">Yes</v-btn>
      <v-btn @click="generateResults">No</v-btn>
   </div>
   <div v-if="wantsToCompose">
      <v-row no-gutters v-for="flashcard in flashcards">
         <v-col sm="6">
            <v-text-field label="Front" variant="outlined" v-model="flashcard.Front"></v-text-field>
         </v-col>
         <v-col sm="6">
            <v-text-field label="Back" variant="outlined" v-model="flashcard.Back"></v-text-field>
         </v-col>
      </v-row>
      <v-btn @click="generateResults" color="primary">Generate</v-btn>
   </div>
</template>

<script lang="ts">
import { useFoundCharsStore } from '@/stores/foundChars'
export default {
   data: () => ({
      flashcards: [{ Front: "", Back: "" }],
      wantsToCompose: false
   }),
   mounted() {
      const store = useFoundCharsStore()
      const storeData = store.getData
      if (storeData == null) this.$router.push('/')
      this.wantsToCompose = false

      this.flashcards.shift()
      //@ts-ignore
      storeData.forEach(element => {
         this.flashcards.push({ Front: element.Front, Back: element.Back })
      });
   },
   methods: {
      async generateResults() {
         console.log(this.flashcards)
      }
   }
}
</script>