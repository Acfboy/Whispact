<script setup lang="ts">
import { computed, onMounted } from "vue";
import { listen } from "@tauri-apps/api/event";
import { error } from '@tauri-apps/plugin-log';
import { useRoute, useRouter } from "vue-router";
import { invoke } from "@tauri-apps/api/core";

const pageName = {
  "home": "Whispact",
  "seal": "时刻",
  "settings": "设置",
  "log": "查看日志"
}

const goBackSet = new Set(["log"]);

const route = useRoute();
const currentName = computed(() => route.name as keyof typeof pageName);

const router = useRouter();
const goBack = () => {
  router.go(-1);
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
      error(event.payload);
      alert("error: " + event.payload);
    });
    await invoke("request_blep_bluetooth_permissions", {});
  } catch (e) {
    alert(e);
  }
});
</script>

<template>
  <v-app>
    <v-app-bar scroll-behavior="elevate">
      <template v-slot:prepend>
        <v-app-bar-nav-icon v-if="!goBackSet.has(currentName)"></v-app-bar-nav-icon>
        <v-icon class="ml-3" v-else @click="goBack">mdi-arrow-left</v-icon>
      </template>
      <v-app-bar-title>{{ pageName[currentName] }}</v-app-bar-title>
    </v-app-bar>

    <v-main>
      <router-view />
    </v-main>


    <v-bottom-navigation grow>
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
