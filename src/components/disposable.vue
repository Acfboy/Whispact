<template>
  <progressLine
  :hight="4"
  :ma="6"
  :duration="500" @endPress="a"></progressLine>
</template>
  
  <script setup>
import { ref } from 'vue'
import  progressLine from "./progress-linear.vue";


const progress = ref(0)
let timer = null
let startTime = 0
let duration = 1000  

function startPress() {
  if (timer) return
  startTime = Date.now()
  progress.value = 0
  timer = setInterval(() => {
    const elapsed = Date.now() - startTime
    progress.value = Math.min((elapsed / duration) * 100, 100)
    if (progress.value >= 100) {
      clearInterval(timer)
      timer = null
    }
  }, 16) 
}

function endPress() {
  if (timer) {
    clearInterval(timer)
    timer = null
  }
  progress.value = 0
}
  </script>