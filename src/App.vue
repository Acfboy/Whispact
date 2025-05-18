<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { info, error } from '@tauri-apps/plugin-log';

async function setDisposableMsg() {
  try {
    await invoke("set_disposable_msg", { msg: "Hello World!" });
  } catch (e) {
    error(e);
    alert(e);
  }
}

const page = ref("home");
const pageName = {
  "home": "Whispact",
  "seal": "时刻",
  "settings": "设置",
}

onMounted(async () => {
  try {
    await listen<string>(
      "recv-disposable-msg",
      (event: { payload: string }) => {
        alert(event.payload);
      }
    );
    await listen<string>("err", (event: { payload: string }) => {
      error(payload);
      alert("error: " + event.payload);
    });
  } catch (e) {
    alert(e);
  }
});
</script>

<template>
  <v-app>
    <v-app-bar scroll-behavior="elevate">
      <template v-slot:prepend>
        <v-app-bar-nav-icon></v-app-bar-nav-icon>
      </template>
      <v-app-bar-title>{{ pageName[page] }}</v-app-bar-title>
    </v-app-bar>

    <v-main>
      <router-view />
    </v-main>

    <v-bottom-navigation grow v-model="page">
      <v-btn value="home" to="/home">
        <v-icon>mdi-home</v-icon>
        <span>主页</span>
      </v-btn>

      <v-btn value="seal" to="/seal">
        <v-icon>mdi-clock-time-four</v-icon>
        <span>时刻</span>
      </v-btn>

      <v-btn value="settings" to="/settings">
        <v-icon>mdi-cog</v-icon>
        <span>设置</span>
      </v-btn>
    </v-bottom-navigation>
  </v-app>
</template>

<style></style>
