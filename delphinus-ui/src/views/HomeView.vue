<template>
  <custom-title headerText="Generate your flashcards!" />
  <v-form @submit.prevent="submitForm" ref="generateForm">
    <v-textarea label="Characters" variant="outlined" v-model="characters" :rules="rules"
      :disabled="loading"></v-textarea>

    <v-radio-group v-model="language" :rules="rules" inline :disabled="loading">
      <v-radio label="Japanese" value="jp"></v-radio>
      <v-radio label="Chinese" value="ch"></v-radio>
    </v-radio-group>

    <v-btn type="submit" :loading="loading" :disabled="loading" block class="mt-2" color="primary">Submit</v-btn>
  </v-form>
</template>

<script lang="ts">
import CustomTitle from "@/components/custom-title.vue";
import { useFoundCharsStore } from '@/stores/foundChars';

export default {
  components: {
    'custom-title': CustomTitle
  },
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
                let somethingFound = false;
                data.forEach((element: { Back: string; }) => {
                  if (element.Back !== 'NOT FOUND') {
                    somethingFound = true
                  }
                });

                if (somethingFound) {
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

<style>
.v-radio-group>.v-input__control {
  align-items: center;
}
</style>