<script setup lang="ts">
import { onMounted, ref } from "vue";
import { invoke, PermissionState } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

const hceUuid = ref("");
const bleMessage = ref("");
const sendMessage = ref("");

async function set_hce() {
  hceUuid.value = await invoke("set_hce_uuid", {});
}

onMounted(async () => {
  try {
    await listen<string>('nfc-new-uuid', (event) => {
      alert(event.payload);
    });
    await listen<string>('nfc-error', (event) => {
      alert('nfc error: ' + event.payload);
    });
    await listen<string>('ble-message-received', (event) => {
      bleMessage.value = event.payload;
    });
  } catch (e) {
    alert(e)
  }
});

async function start_read() {
  invoke('start_reader', {});
  alert('invoked');
}

async function start_peripheral() {
  try {
    await invoke("start_ble_peripheral", {});
  } catch (e) {
    alert(e)
  }
}

async function start_central() {
  try {
    await invoke("start_ble_central_with_uuid", { uuid: "0000ffe1-0000-1000-8000-00805f9b34fb" });
  } catch (e) {
    alert(e);
  }
}

async function blep_permission() {
  try {
    const permission = await invoke<PermissionState>('request_blep_bluetooth_permissions')
    alert(JSON.stringify(permission));
  } catch (e) {
    alert(e);
  }
}

async function central_send() {
  await invoke("ble_central_send", { msg: sendMessage.value });
}
</script>

<template>
  <main class="container">
    <h1>Welcome to Whispact! 这是测试页面。</h1>
    <div>BLE Recv: {{ bleMessage }}</div>
    <div>HCE uuid: {{ hceUuid }}</div>
    <button @click="set_hce">SET HCE</button>
    <button @click="start_central"> START CENTRAL</button>
    <button @click="start_peripheral">START PERIPHERAL</button>
    <input type="text" v-model="sendMessage" />
    <button @click="central_send">CENTRAL SEND</button>
    <button @click="start_read">START READER</button>
    <button @click="blep_permission">BLEP PERMISSION</button>
  </main>
</template>
