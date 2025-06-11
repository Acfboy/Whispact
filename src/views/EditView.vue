<template>
  <v-container>
    <v-text-field :label="titleLabel" variant="outlined" v-model="textTitle"></v-text-field>
    <v-textarea :label="bodyLabel" variant="solo" v-model="textBody"></v-textarea>
  </v-container>
</template>

<script setup lang="ts">
import { try_invoke } from '@/utils/utils';
import { ref } from 'vue'
import { onBeforeRouteLeave, useRoute } from 'vue-router';
import { MessageDraft } from '../types'  // 添加类型导入

const props = defineProps({
  id: {
    type: String,
    default: "",
  },
  type: {
    type: String,
    validator: (value: string) => ["Plan", "Mail", "Disposable"].includes(value),
  },
});

const titleLabelMap = {
  "Disposable": "输入消息备注",
  "Plan": "输入事件标题",
  "Mail": "输入信的标题",
};
const titleLabel = ref(titleLabelMap[props.type as keyof typeof titleLabelMap]);

const bodyLableMap = {
  "Disposable": "输入一次性消息",
  "Plan": "输入事件描述",
  "Mail": "输入信的正文",
}
const bodyLabel = ref(bodyLableMap[props.type as keyof typeof bodyLableMap]);

const route = useRoute();
const textBody = ref<string>(route.query.body as string || "");
const textTitle = ref<string>(route.query.title as string || "");



onBeforeRouteLeave(async () => {
  if (props.type == "Plan") {
    let data: { drafts: object } = (await try_invoke("load_plan_drafts"))!;
    let drafts = new Map(Object.entries(data.drafts));
    drafts.set(props.id, { title: textTitle.value, body: textBody.value });
    await try_invoke("store_plan_drafts", { data: { drafts } });
  }
  else if (props.type == "Disposable") {
    const result = await invoke("load_disposable_drafts") as { drafts: MessageDraft[] };
    const drafts = result.drafts || [];
    
    if (textTitle.value || textBody.value) {
      drafts.unshift({
        title: textTitle.value || '无标题',
        body: textBody.value
      });
      await invoke("store_disposable_drafts", { 
        data: { drafts } 
      });
    }
  }
});

</script>