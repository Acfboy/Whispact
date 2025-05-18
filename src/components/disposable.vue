<template>
  <progressLine
  :hight="4"
  :ma="6"></progressLine>
</template>
  
  <script setup>
import { ref } from 'vue'
import  progressLine from "./progress-linear.vue";


const progress = ref(0)
let timer = null
let startTime = 0
let duration = 1000  

  </script>