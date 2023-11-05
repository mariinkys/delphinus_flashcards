<template>
   <h1>Generated!</h1>
   <v-btn @click="copyToClipboard" color="primary">Copy Result</v-btn>
</template>

<script lang="ts">
import { useResultStore } from '@/stores/result'
export default {
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