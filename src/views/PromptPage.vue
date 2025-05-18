<template>
  <v-app>
      <v-container fluid>
        <v-row justify="center">
         <b> 时光胶囊 </b>
        </v-row>
      </v-container>
      <v-textarea label="属于你们的打卡瞬间" counter auto-grow class="ma-6"></v-textarea>
      <div>
      <v-btn @click="add-textarea">
        <v-icon>mid-plus-thick</v-icon>
      </v-btn>
      </div>
  </v-app>
</template>

<script setup="ts">
import { ref } from "vue";

</script>