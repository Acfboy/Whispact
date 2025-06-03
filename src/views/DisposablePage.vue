<template>
  <v-container>
    <v-row>
      <v-col cols="12" md="8" class="mx-auto">
        <!-- 消息进度条列表，只显示标题 -->
        <div
          v-for="(draft, idx) in drafts"
          :key="draft.title"
          class="mb-2"
          style="width: 100%;"
        >
          <progressLine
            :hight="4"
            :ma="6"
            :duration="1000"
            :payload="idx"
            @endPress="preview"
            @click="addToBuffer"
          >
            <div class="d-flex flex-column">
              <span class="text-subtitle-1">{{ draft.title }}</span>
            </div>
          </progressLine>
        </div>

        <!-- 空状态时显示默认的进度条 -->
        <template v-if="drafts.length === 0">
          <div style="width: 100%;">
            <progressLine :hight="4" :ma="6">这里还没有消息噢</progressLine>
          </div>
        </template>

        <!-- 加号输入框（不传 payload，不绑定 endPress） -->
        <div class="mb-2" style="width: 100%;">
          <MessageInput @new-message="addMessage" />
        </div>

        <!-- 预览内容弹窗 -->
        <v-dialog v-model="previewDialog" max-width="500px">
          <v-card v-if="previewIndex !== null && drafts[previewIndex]">
            <v-card-title>
              {{ drafts[previewIndex].title }}
              <v-spacer></v-spacer>
              <v-btn icon="mdi-close" variant="text" @click="closePreview"></v-btn>
            </v-card-title>
            <v-card-text>
              {{ drafts[previewIndex].body }}
            </v-card-text>
          </v-card>
        </v-dialog>
      </v-col>
    </v-row>
  </v-container>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import progressLine from "../components/progress-linear.vue"
import MessageInput from "../components/MessageInput.vue"
import { MessageDraft } from '../types'
import { invoke } from '@tauri-apps/api/core'

const drafts = ref<MessageDraft[]>([])

async function saveDrafts() {
  await invoke('store_disposable_drafts', { data: { drafts: drafts.value } })
}

async function loadDrafts() {
  try {
    const result = await invoke('load_disposable_drafts') as { drafts: MessageDraft[] }
    drafts.value = result.drafts || []
  } catch (e) {
    drafts.value = []
  }
}

const addMessage = async (message: { title: string; content: string }) => {
  const draft: MessageDraft = {
    title: message.title,
    body: message.content
  }
  drafts.value.unshift(draft)
  await saveDrafts()
}

const previewDialog = ref(false)
const previewIndex = ref<number | null>(null)

const preview = (payload: unknown) => {
  if (typeof payload === 'number' && payload >= 0 && payload < drafts.value.length) {
    previewIndex.value = payload
    previewDialog.value = true
  }
}

const closePreview = () => {
  previewDialog.value = false
  previewIndex.value = null
}

const deleteDraft = async (title: string) => {
  drafts.value = drafts.value.filter(draft => draft.title !== title)
  await saveDrafts()
}

const addToBuffer = async (payload: unknown) => {
  if (typeof payload === 'number' && payload >= 0 && payload < drafts.value.length) {
    const selectedDraft = drafts.value[payload];
    try {
      await invoke('set_disposable_msg', { msg: selectedDraft.body });
    } catch (e) {
      console.error('Failed to set message:', e);
    }
  }
}

onMounted(() => {
  loadDrafts()
})
</script>

<style scoped>
.mb-2 {
  margin-bottom: 8px;
}
</style>
