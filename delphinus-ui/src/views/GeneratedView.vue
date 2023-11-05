<template>
   <custom-title header-text="Flashcards Generated!" />
   <v-btn @click="copyToClipboard" color="primary" block class="mt-3">Copy Result</v-btn>
</template>

<script lang="ts">
import CustomTitle from "@/components/custom-title.vue";
import { useResultStore } from '@/stores/result'

export default {
   components: {
      'custom-title': CustomTitle
   },
   data: () => ({
      result: ''
   }),
   mounted() {
      const store = useResultStore()
      const storeData = store.getData
      if (storeData == null || storeData == '') this.$router.push('/')

      this.result = storeData
   },
   methods: {
      copyToClipboard() {
         try {
            navigator.clipboard.writeText(this.result);
            this.$toast.open({
               message: 'Copied to Clipboard!',
               type: 'success',
            });
         } catch (error) {
            console.error('Unable to copy to clipboard:', error);
            this.$toast.open({
               message: 'Failed to copy to clipboard.',
               type: 'error',
            });
         }
      }
   }
}
</script>