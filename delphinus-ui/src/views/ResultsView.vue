<template>
   <custom-title headerText="Results Found!" />
   <div v-if="!wantsToCompose">
      <p class="text-center text-body-1">Do you want to revise the flashcards?</p>
      <div class="text-center mt-3">
         <v-btn @click="wantsToCompose = true" class="mx-2" color="success">Yes</v-btn>
         <v-btn @click="generateResults" class="mx-2" color="error">No</v-btn>
      </div>
   </div>
   <div v-if="wantsToCompose">
      <div v-for="(flashcard, index) in flashcards" :key="index" class="row">
         <div class="col">
            <v-text-field label="Front" variant="outlined" v-model="flashcard.Front" :disabled="loading"></v-text-field>
         </div>
         <div class="col">
            <v-text-field label="Back" variant="outlined" v-model="flashcard.Back" :disabled="loading"></v-text-field>
         </div>
         <div class="col smaller">
            <v-btn @click="deleteFlashcard(index)" prepend-icon="mdi-delete-circle" color="red"
               :disabled="loading">DELETE</v-btn>
         </div>
      </div>
      <div class="text-center mt-5">
         <v-btn @click="generateResults" color="primary" :disabled="loading" block>Generate</v-btn>
      </div>
   </div>
</template>

<script lang="ts">
import CustomTitle from "@/components/custom-title.vue";
import { useFoundCharsStore } from '@/stores/foundChars'
import { useResultStore } from '@/stores/result'

export default {
   components: {
      'custom-title': CustomTitle
   },
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

<style>
div.v-input__details {
   display: none;
}

.row {
   margin-top: 3vh;
   display: flex;
   flex-direction: column;
   align-self: center;
   box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
   border-radius: 10px;
}

.col {
   flex: 1;
   margin: 10px;
}

.smaller {
   flex: 0.1;
   align-self: center;
}

@media (min-width: 600px) {
   .row {
      flex-direction: row;
   }
}

@media (max-width: 600px) {
   .smaller {
      flex: 1;
   }
}
</style>