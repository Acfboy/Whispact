<template>
  <v-container>
    <v-card variant="flat" title="开发人员选项">
      <v-card-text>
        <v-btn variant="outlined" @click="setDisposableMsg" class="ma-1">
          测试通信
        </v-btn>
        
        <v-btn variant="outlined" @click="testCentral" class="ma-1">
          测试主端
        </v-btn>
        <v-btn variant="outlined" @click="navigateTo('log')" class="ma-1">
          查看日志
        </v-btn>
      </v-card-text>
    </v-card>
  </v-container>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { error } from "@tauri-apps/plugin-log";
import { useRouter } from "vue-router";

async function setDisposableMsg() {
  try {
    await invoke("set_disposable_msg", { msg: "Hello World!" });
  } catch (e: unknown) {
    error(String(e));
    alert(e);
  }
}

async function testCentral() {
  try {
    await invoke("test_ble_central", { msg: "Hello World!" });
  } catch (e: unknown) {
    error(String(e));
    alert(e);
  }
}

const router = useRouter();

const navigateTo = (routerName: string) => {
  router.push({ name: routerName });
}

</script>