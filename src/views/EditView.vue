<template>
  <v-container>
    <v-text-field :label="titleLabel" variant="outlined" v-model="textTitle"></v-text-field>
    <v-textarea :label="bodyLabel" variant="solo" v-model="textBody"></v-textarea>
  </v-container>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { ref } from 'vue'
import { onBeforeRouteLeave, useRoute } from 'vue-router';

const props = defineProps({
  id: {
    type: String,
    default: "",
  },
  type: {
    type: String,
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
const textBody = ref(route.query.body || "");
const textTitle = ref(route.query.title || "");

onBeforeRouteLeave(async () => {
  if (props.type == "Plan") {
    let data: { drafts: {} } = await invoke("load_plan_drafts");
    let drafts = new Map(Object.entries(data.drafts));
    drafts.set(props.id, { title: textTitle.value, body: textBody.value });
    await invoke("store_plan_drafts", { data: { drafts } });
  }
});

</script>