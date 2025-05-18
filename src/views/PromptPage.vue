<template>
  <v-container fluid>
        <v-row justify="center">
         <b> 时光胶囊 </b>
        </v-row>
 </v-container>
   <v-container fluid>
     <v-textarea label="属于你们的打卡瞬间" counter auto-grow class="ma-6"></v-textarea>
        <v-row justify="center" class="ma-5">
           <v-btn @click="add-textarea" rounded block>确定</v-btn>
        </v-row>
 </v-container>
</template>

<script setup="ts">
import { ref } from "vue";


</script>