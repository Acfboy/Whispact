<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { listen } from "@tauri-apps/api/event";
import { error } from '@tauri-apps/plugin-log';
import { useRoute, useRouter } from "vue-router";
import { invoke } from "@tauri-apps/api/core";
// import { navigateTo } from "./utils";

const pageName = {
  "home": "Whispact",
  "plan": "时刻",
  "settings": "设置",
  "log": "查看日志",
  "prompt": "打卡瞬间",
  "disposable": "一次性消息",
  "mailbox": "信箱",
  "seal": "打卡此刻"
}

const goBackSet = new Set(["log", "prompt", "disposable", "settings", "seal"]);

const route = useRoute();
const currentName = computed(() => route.name as keyof typeof pageName);


const router = useRouter();
const goBack = () => {
  router.go(-1);
}
const navigateTo = (routeName: string) => {
  setTimeout(() => {
    router.push({ name: routeName });
  }, 100);
};


const errorMsg = ref("");
const errorBar = ref(false);

const drawer = ref(false);

onMounted(async () => {
  await listen<string>(
    "recv-disposable-msg",
    (event: { payload: string }) => {
      alert(event.payload);
    }
  );
  await listen<object>("err", (event: { payload: object }) => {
    error(JSON.stringify(event.payload));
    errorMsg.value = JSON.stringify(event.payload);
    errorBar.value = true;
  });
  await invoke("request_blep_bluetooth_permissions", {});
});
</script>

<template>
  <v-app>
    <v-app-bar scroll-behavior="elevate">
      <template v-slot:prepend>
        <v-app-bar-nav-icon v-if="!goBackSet.has(currentName)" @click.stop="drawer = !drawer"></v-app-bar-nav-icon>
        <v-icon class="ml-3" v-else @click="goBack">mdi-arrow-left</v-icon>
      </template>
      <v-app-bar-title>{{ pageName[currentName] }}</v-app-bar-title>
    </v-app-bar>

    <v-navigation-drawer v-model="drawer" temporary>
      <v-list>
        <v-list-item :active="false" color="primary" key="_" value="_" @click="navigateTo('settings')">
          <template v-slot:prepend>
            <v-icon icon="mdi-cog"></v-icon>
          </template>

          <v-list-item-title>设置</v-list-item-title>
        </v-list-item>
      </v-list>
    </v-navigation-drawer>

    <v-main>
      <router-view />
    </v-main>

    <v-bottom-navigation grow>
      <v-btn value="home" to="/home">
        <v-icon>mdi-home</v-icon>
        <span>主页</span>
      </v-btn>

      <v-btn value="plan" to="/plan">
        <v-icon>mdi-clock-time-four</v-icon>
        <span>时刻</span>
      </v-btn>

      <v-btn value="mailbox" to="/mailbox">
        <v-icon>mdi-email-outline</v-icon>
        <span>信箱</span>
      </v-btn>
    </v-bottom-navigation>

    <v-snackbar v-model="errorBar" multi-line>
      {{ "错误：" + errorMsg }}

      <template v-slot:actions>
        <v-btn color="red" variant="text" @click="errorBar = false">
          关闭
        </v-btn>
      </template>
    </v-snackbar>
  </v-app>
</template>

<style></style>
