<template>
   <v-footer>
      <v-row justify="center" no-gutters>
         <v-btn variant="text" class="mx-2" icon="mdi-mastodon" href="https://mastodon.social/@mariinkys"
            target="_blank"></v-btn>
         <v-btn variant="text" class="mx-2" icon="mdi-github" href="https://github.com/mariinkys/delphinus_flashcards"
            target="_blank"></v-btn>
         <v-btn variant="text" class="mx-2" icon="mdi-coffee" href="https://www.buymeacoffee.com/mariinkys"
            target="_blank"></v-btn>
         <v-col class="text-center mt-4" cols="12">
            {{ new Date().getFullYear() }} — <strong>Alex Marín</strong> — FLOSS, now and always!
         </v-col>
      </v-row>
   </v-footer>
</template>