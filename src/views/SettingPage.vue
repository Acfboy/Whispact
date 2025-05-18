<template>
  <v-container>
    <v-card variant="flat" title="开发人员选项">
      <v-card-text>
        <v-btn variant="outlined" @click="at">
          测试通信
        </v-btn>
      </v-card-text>
    </v-card>
  </v-container>
</template>

<script lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { defineComponent } from "vue";

let at = async () => {
  await invoke("set_disposable_msg", { msg: "Hello World!" });
}

export default defineComponent({
  setup() {
    return {
      at
    };
  },
});
</script>