<script setup lang="ts">
import { onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";


async function setDisposableMsg() {
  try {
    await invoke("set_disposable_msg", { msg: "Hello World!" });
  } catch (e) {
    alert(e)
  }
}

onMounted(async () => {
  try {
    await listen<string>('recv-disposable-msg', (event: { payload: string; }) => {
      alert(event.payload);
    });
    await listen<string>('err', (event: { payload: string; }) => {
      alert('error: ' + event.payload);
    });
  } catch (e) {
    alert(e)
  }
});
</script>

<template>
  <v-app>
    <v-main>
      <main class="container">
        <h1>Welcome to Whispact! 这是测试页面。</h1>
        <v-btn @click="setDisposableMsg">测试消息</v-btn>
      </main>
    </v-main>

    <v-bottom-navigation grow>
      <v-btn value="seal" to="/seal">
        <v-icon>mdi-clock-time-four</v-icon>
        <span>Seal</span>
      </v-btn>

      <v-btn value="home" to="/home">
        <v-icon>mdi-home</v-icon>
        <span>Home</span>
      </v-btn>

      <v-btn value="settings" to="/settings">
        <v-icon>mdi-cog</v-icon>
        <span>Settings</span>
      </v-btn>
    </v-bottom-navigation>
  </v-app>
</template>

<style></style>