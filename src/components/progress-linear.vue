<template>
  <div>
    <v-card class="`ma-${ma}`">
      <v-progress-linear :model-value="progress" :height="height" @mousedown="handleStart" @mouseup="handleEnd"
        @mouseleave="handleEnd" @touchstart.prevent="handleStart" @touchend="handleEnd">
      </v-progress-linear>
      <v-card-text @mousedown="handleStart" @mouseup="handleEnd" @mouseleave="handleEnd"
        @touchstart.prevent="handleStart" @touchend="handleEnd">
        <slot />
        {{ props.message }}
      </v-card-text>
    </v-card>
  </div>
</template>

<script setup>
import { ref } from 'vue'

const props = defineProps({
  ma: {
    type: Number,
    default: 6
  },
  height: {
    type: Number,
    default: 2
  },
  duration: {
    type: Number,
    default: 1000
  },
  message: {
    type: String,
    default: ""
  }
})
const emit = defineEmits(['endPress'])

const progress = ref(0)
let timer = null
let startTime = 0

function handleStart() {
  if (timer) return
  startTime = Date.now()
  progress.value = 0
  timer = setInterval(() => {
    const elapsed = Date.now() - startTime
    progress.value = Math.min((elapsed / props.duration) * 100, 100)
    if (progress.value >= 100) {
      clearInterval(timer)
      timer = null
    }
  }, 16)
}

function handleEnd() {
  if (timer) {
    clearInterval(timer)
    timer = null
  }
  progress.value = 0
  emit('endPress')
}
</script>