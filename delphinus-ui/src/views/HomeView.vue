<template>
  <h1>Generate your flashcards!</h1>
  <v-form @submit.prevent="submitForm" ref="generateForm">
    <v-textarea label="Characters" variant="outlined" v-model="characters" :rules="rules"
      :disabled="loading"></v-textarea>
    <v-radio-group v-model="language" :rules="rules" inline :disabled="loading">
      <v-radio label="Japanese" value="jp"></v-radio>
      <v-radio label="Chinese" value="ch"></v-radio>
    </v-radio-group>
    <v-btn type="submit" :loading="loading" :disabled="loading" block class="mt-2">Submit</v-btn>
  </v-form>
</template>

<script lang="ts">
import { useFoundCharsStore } from '@/stores/foundChars'
export default {
  data: () => ({
    loading: false,
    characters: '',
    language: '',
    rules: [
      (value: any) => {
        if (value) return true
        return 'Required.'
      },
    ],
  }),
  watch: {
    loading(val) {
      if (!val) return
    }
  },
  methods: {
    async submitForm() {
      //@ts-ignore
      const { valid } = await this.$refs.generateForm.validate()
      if (valid) {
        try {
          this.loading = true
          const formData = new URLSearchParams();
          formData.append('characters', this.characters)
          formData.append('language', this.language)

          const url = import.meta.env.VITE_API_ENDPOINT + "/searchdictionary"
          await fetch(url, {
            method: "POST",
            mode: "cors",
            cache: "no-cache",
            headers: {
              "Content-Type": "application/x-www-form-urlencoded"
            },
            body: formData
          })
            .then(data => data.json())
            .then(data => {
              if (data != null) {
                const store = useFoundCharsStore()
                store.updateData(data)
                this.loading = false;
                this.$router.push('results')
              } else {
                this.$toast.open({
                  message: 'No results found',
                  type: 'warning'
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
      }
    }
  }
}
</script>