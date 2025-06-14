<template>
    <v-dialog v-model="dialog" max-width="500px">
      <template v-slot:activator="{ props }">
        <v-card class="ma-6"  v-bind="props" >
          <v-progress-linear
            :height="4"
            :model-value="0"
          ></v-progress-linear>
          <v-card-text class="d-flex justify-center align-center">
            <v-icon icon="mdi-plus" size="large"></v-icon>
          </v-card-text>
        </v-card>
      </template>
  
      <v-card class="message-input">
        <v-card-title class="text-h6 pa-4">
          新建一次性消息
          <v-spacer></v-spacer>
          <v-btn
            icon="mdi-close"
            variant="text"
            @click="dialog = false"
          ></v-btn>
        </v-card-title>
  
        <v-card-text class="pa-4">
          <v-text-field
            v-model="title"
            label="标题"
            variant="outlined"
            density="comfortable"
            class="mb-2"
            @keyup.enter="sendMessage"
          ></v-text-field>
          <v-textarea
            v-model="body"
            label="内容"
            variant="outlined"
            rows="4"
            auto-grow
            class="mb-2"
          ></v-textarea>
        </v-card-text>
  
        <v-card-actions class="pa-4">
          <v-spacer></v-spacer>
          <v-btn
            color="primary"
            @click="sendMessage"
            :disabled="!isValid"
          >
            发送
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
</template>
  
<script setup lang="ts">
import { ref, computed } from 'vue'
// import { MessageDraft } from '../types'

const dialog = ref(false)
const emit = defineEmits<{
  (e: 'new-message', message: { title: string; content: string }): void
}>()

const title = ref('')
const body = ref('')

const isValid = computed(() => {
  return title.value.trim() !== '' && body.value.trim() !== ''
})

const sendMessage = () => {
  if (!isValid.value) return

  emit('new-message', {
    title: title.value.trim(),
    content: body.value.trim()
  })
  
  // 清空输入并关闭对话框
  title.value = ''
  body.value = ''
  dialog.value = false
}
</script>
  
<style scoped>
.message-input {
  max-width: 500px;
}
</style>