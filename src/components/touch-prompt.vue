<template>
  <v-dialog v-model="localSyncTouch" width="auto">
    <v-card class="d-flex flex-column items-center justify-center">
      <div class="d-flex justify-center">
        <v-icon size="30vw">
          mdi-access-point
        </v-icon>
      </div>
      <v-card-text class="text-center">
        {{ '触碰两台手机背面，' + prompt }}
      </v-card-text>
    </v-card>
  </v-dialog>
</template>

<script setup>
import { ref, watch } from 'vue'

const props = defineProps({
  syncTouch: {
    type: Boolean,
    default: false
  },
  prompt: {
    type: String,
    default: ""
  },
})

const emit = defineEmits(['update:syncTouch'])

const localSyncTouch = ref(props.syncTouch)

watch(localSyncTouch, (newValue) => {
  emit('update:syncTouch', newValue)
})

watch(() => props.syncTouch, (newValue) => {
  localSyncTouch.value = newValue
})
</script>