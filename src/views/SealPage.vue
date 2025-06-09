<template>
  <v-container class="fill-height justify-center align-items ">
    <v-text-field variant="solo-filled" hint="输入要打卡的此刻，点击按钮后相碰确认。" append-inner-icon="mdi-send"
      @click:append-inner="onClick" persistent-hint v-model="msg"></v-text-field>

    <touchPrompt v-model:syncTouch="syncTouch" prompt="完成打卡。"></touchPrompt>

    <v-snackbar v-model="errorBar" multi-line>
      两边打卡内容不一致，请重试。
    </v-snackbar>

    <v-snackbar v-model="success" multi-line color="success">
      打卡成功。
      <template v-slot:actions>
        <v-btn color="success" variant="text" @click="jumpToPlan">
          到时刻页查看
        </v-btn>
      </template>
    </v-snackbar>
  </v-container>
</template>

<script setup lang="ts">
import { ref, watchEffect } from "vue";
import touchPrompt from "@/components/touch-prompt.vue"
import { useRouter } from "vue-router";
import { listen } from "@tauri-apps/api/event";
import { Instance, SealedInstances } from "@/types";
import { try_invoke } from "@/utils/utils";

const msg = ref("");
const syncTouch = ref(false);
const errorBar = ref(false);
const success = ref(false);

const onClick = async () => {
  try_invoke("set_seal_msg", { msg: msg.value });
  syncTouch.value = true;
}

const jumpToPlan = () => {
  sessionStorage.setItem("tab", "Finished");
  let router = useRouter();
  router.push({ name: "plan" })
}

const onTouch = async (event: { payload: string }) => {
  if (syncTouch.value == false)
    return;

  if (event.payload != msg.value) {
    syncTouch.value = false;
    errorBar.value = true;
  } else {
    let sealed: SealedInstances = (await try_invoke("load_sealed_instances", {}))!;
    const d = new Date();
    const timestamp = d.toLocaleDateString() + ' ' + d.toLocaleTimeString();
    const instance: Instance = { time: timestamp, instance: msg.value };
    sealed.instances.push(instance);
    await try_invoke("store_sealed_instances", { data: sealed });
    syncTouch.value = false;
    success.value = true;
  }
}

watchEffect(async () => {
  if (syncTouch.value == false) 
    await try_invoke("clear_msg", {});
});

(async () => {
  await listen<string>("recv-seal-msg", onTouch);
})();
</script>