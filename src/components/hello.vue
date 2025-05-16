<template>
  <v-app>
    <v-main>
      <v-container fluid>
        <v-row justify="center" align="center">
          <v-col cols="12" md="8" lg="6">请与另一个手机碰一碰</v-col>
        </v-row></v-container>
    </v-main>
  </v-app>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
    name: "LoadingComponent",
    components: {
    },
    emits: [],
    setup() {
        return {
            
        };
    },
});
</script>

<style>
  
</style>