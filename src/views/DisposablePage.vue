<template>
  <v-container class="mt-0 pt-0">
    <v-btn class="ma-2 ml-0" variant="tonal" size="small" append-icon="mdi-plus" @click="navigateToEdit">
      新消息
    </v-btn>
    <v-card class="scroll-container" variant="text">
      <div v-for="(draft, idx) in drafts" :key="draft.title" class="mb-2 position-relative" style="width: 100%;">
        <progressLine :hight="4" :ma="6" :duration="500" :payload="idx" @endPress="startTransfer(idx)">
          <div class="d-flex flex-column">
            <div class="d-flex justify-space-between align-center">
              <span class="text-subtitle-1">{{ draft.title }}</span>
            </div>
          </div>
        </progressLine>
        <div class="action-buttons">
          <v-btn icon="mdi-eye-outline" variant="text" size="small" color="primary" class="transfer-btn"
            @click.stop="preview(idx)"></v-btn>
          <v-btn icon="mdi-delete" variant="text" size="small" color="error" class="delete-btn"
            @click.stop="deleteDraft(idx)"></v-btn>
        </div>
      </div>
    </v-card>

    <!-- 预览内容弹窗 -->
    <v-dialog v-model="previewDialog" max-width="500px">
      <v-card v-if="previewIndex !== null && drafts[previewIndex]">
        <v-card-title>
          {{ drafts[previewIndex].title }}
        </v-card-title>
        <v-card-text>
          {{ drafts[previewIndex].body }}
        </v-card-text>
      </v-card>
    </v-dialog>

    <!-- 添加碰一碰提示组件 -->
    <touch-prompt v-model="touching" :prompt="touchingPrompt"></touch-prompt>

    <!-- 添加成功/失败提示 -->
    <v-snackbar :timeout="2000" color="green-lighten-3" v-model="transferSuccess">
      一次性消息已传递
    </v-snackbar>
  </v-container>
</template>

<script setup lang="ts">
import { ref, onMounted, watchEffect } from 'vue'
import progressLine from "@/components/progress-linear.vue"
import touchPrompt from "@/components/touch-prompt.vue"
import { onBeforeRouteLeave, useRouter } from 'vue-router'
import { MessageDraft } from '@/types'
import { try_invoke } from '@/utils/utils'
import { listen, UnlistenFn } from '@tauri-apps/api/event'

const drafts = ref<MessageDraft[]>([])
const router = useRouter()

const navigateToEdit = () => {
  router.push({
    name: 'edit',
    params: {
      type: 'Disposable',
      id: String(drafts.value.length)
    }
  })
}

async function saveDrafts() {
  await try_invoke('store_disposable_drafts', { data: { drafts: drafts.value } })
}

async function loadDrafts() {
  try {
    const result = await try_invoke('load_disposable_drafts') as { drafts: MessageDraft[] }
    drafts.value = result.drafts || []
  } catch {
    drafts.value = []
  }
}
const deleteDraft = async (index: number) => {
  if (index >= 0 && index < drafts.value.length) {
    drafts.value.splice(index, 1)
    await saveDrafts()
  }
}


const previewDialog = ref(false)
const previewIndex = ref<number | null>(null)

const preview = (idx: number) => {
  if (idx >= 0 && idx < drafts.value.length) {
    previewIndex.value = idx
    previewDialog.value = true
  }
}

const touching = ref(false)
const touchingPrompt = ref("")
const transferSuccess = ref(false)
const transferringIndex = ref<number | null>(null)

const startTransfer = async (idx: number) => {
  if (idx >= 0 && idx < drafts.value.length) {
    const selectedDraft = drafts.value[idx];
    await try_invoke('set_disposable_msg', { msg: selectedDraft.body });
    touchingPrompt.value = "传递一次性消息";
    touching.value = true;
    transferringIndex.value = idx;
  }
}

let unlisten: undefined | UnlistenFn = undefined;

(async () => {
  unlisten = await listen("touching", async () => {
    if (touching.value && transferringIndex.value !== null) {
      deleteDraft(transferringIndex.value);
      touching.value = false;
      transferringIndex.value = null;
      transferSuccess.value = true;
    }
  });
})();

watchEffect(async () => {
  if (touching.value == false) {
    await try_invoke('clear_msg', {});
    transferringIndex.value = null;
  }
});

onMounted(() => {
  loadDrafts()
})

onBeforeRouteLeave(() => {
  if (unlisten)
    unlisten();
});
</script>

<style scoped>
.mb-2 {
  margin-bottom: 8px;
}

.action-buttons {
  position: absolute;
  right: 5px;
  top: 50%;
  transform: translateY(-50%);
  z-index: 1;
  display: flex;
  gap: 4px;
}

.transfer-btn {
  opacity: 0.8;
  transition: opacity 0.2s;
}

.transfer-btn:hover {
  opacity: 1;
}

.delete-btn {
  position: absolute;
  right: 35px;
  top: 50%;
  transform: translateY(-50%);
  z-index: 1;
}

.add-btn {
  margin-top: 10px;
}

.scroll-container {
  overflow-y: auto;
  max-height: 75vh;
  padding-top: 0;
}
</style>
