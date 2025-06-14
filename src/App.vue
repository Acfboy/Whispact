<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { listen } from "@tauri-apps/api/event";
import { error } from '@tauri-apps/plugin-log';
import { useRoute, useRouter } from "vue-router";
import { getTimeStamp, timeStampUuid, try_invoke } from "./utils/utils";
import { Mail, MailCover, MailCoverList } from "./types";

const pageName = {
  "home": "Whispact",
  "plan": "时刻",
  "settings": "设置",
  "log": "查看日志",
  "prompt": "打卡瞬间",
  "disposable": "一次性消息",
  "mailbox": "信箱",
  "seal": "打卡此刻",
  "edit": "编辑",
  "read": "内容"
}

const goBackSet = new Set(["log", "prompt", "disposable", "settings", "seal", "edit", "read"]);

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

(async () => {
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
})();


const recvMail = ref(false);
listen("recv-mail", async (event: { payload: Mail }) => {
  const uuid = timeStampUuid();
  const inner = event.payload.inner;
  const cover: MailCover = { cover: event.payload.cover, sealed: true, timestamp: getTimeStamp() };
  let covers: MailCoverList = { mails: new Map(Object.entries((await try_invoke<{ mails: object }>("load_mail_covers"))!.mails)) };
  covers.mails.set(uuid, cover);
  await try_invoke("store_mail_covers", { data: covers });
  await try_invoke("store_mail_inner", { uuid, data: inner });
  recvMail.value = true;
});

const disposablePrompt = ref(false);
listen("recv-disposable-msg", (event: { payload: string }) => {
  router.push({ name: "read", query: { title: "一次性消息", body: event.payload } });
  disposablePrompt.value = true;
});

onMounted(async () => {
  await try_invoke("request_blep_bluetooth_permissions", {});
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
        <v-icon>mdi-email</v-icon>
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

    <v-snackbar :timeout="2000" color="green-lighten-3" v-model="recvMail">
      收到一封信
    </v-snackbar>
    
    <v-snackbar :timeout="2000" color="red-lighten-3" v-model="disposablePrompt">
      注意，离开预览界面这条永远丢失。
    </v-snackbar>
  </v-app>
</template>

<style></style>
