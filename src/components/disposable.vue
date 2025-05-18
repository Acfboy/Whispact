<template>
  <!-- <div>
    <v-text-field
      :rules="rules"
      hide-details="auto"
      label="请输入消息"
    ></v-text-field>
  </div> -->
  <div>
  <v-card class="ma-6">
    <v-progress-linear
      :model-value="progress"
      :height="2"
       @mousedown="startPress"
       @mouseup="endPress"
       @mouseleave="endPress"
       @touchstart.prevent="startPress"
       @touchend="endPress"
       >
  </v-progress-linear>
      <v-card-text
       @mousedown="startPress"
       @mouseup="endPress"
       @mouseleave="endPress"
       @touchstart.prevent="startPress"
       @touchend="endPress">
          Lorem scaevola imperdiet nec ut, sed euismod convenire principes at. Est et nobis iisque percipit.
      </v-card-text>
  </v-card>
  </div>
  <progressLine
  :hight="4"
  :ma="6"
  :duration="2000" @endPress="a"></progressLine>
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