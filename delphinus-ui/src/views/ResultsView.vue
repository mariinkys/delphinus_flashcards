<template>
   <h1>Results Found!</h1>
   <div v-if="!wantsToCompose">
      <p>Do you want to revise the flashcards?</p>
      <v-btn @click="wantsToCompose = true">Yes</v-btn>
      <v-btn @click="generateResults">No</v-btn>
   </div>
   <div v-if="wantsToCompose">
      <v-row no-gutters v-for="(flashcard, index) in flashcards" :key="index">
         <v-col sm="5">
            <v-text-field label="Front" variant="outlined" v-model="flashcard.Front" :disabled="loading"></v-text-field>
         </v-col>
         <v-col sm="5">
            <v-text-field label="Back" variant="outlined" v-model="flashcard.Back" :disabled="loading"></v-text-field>
         </v-col>
         <v-col sm="2">
            <v-btn @click="deleteFlashcard(index)" icon="mdi-delete-circle" color="red"></v-btn>
         </v-col>
      </v-row>
      <v-btn @click="generateResults" color="primary" :disabled="loading">Generate</v-btn>
   </div>
</template>

<script lang="ts">
import { useFoundCharsStore } from '@/stores/foundChars'
import { useResultStore } from '@/stores/result'
export default {
   data: () => ({
      loading: false,
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
         try {
            this.loading = true

            const url = import.meta.env.VITE_API_ENDPOINT + "/generateflashcards"
            await fetch(url, {
               method: "POST",
               mode: "cors",
               cache: "no-cache",
               headers: {
                  "Content-Type": "application/json"
               },
               body: JSON.stringify(this.flashcards)
            })
               .then(data => data.json())
               .then(data => {
                  if (data != null) {
                     this.loading = false;
                     const store = useResultStore()
                     store.updateData(data)
                     this.$router.push('generated')
                  } else {
                     this.$toast.open({
                        message: 'Something went wrong!',
                        type: 'error',
                     });
                     this.loading = false;
                  }
               })
         } catch (error) {
            this.$toast.open({
               message: 'Something went wrong!',
               type: 'error',
            });
            this.loading = false;
         }
      },
      deleteFlashcard(index: number) {
         this.flashcards.splice(index, 1);
      }
   }
}
</script>