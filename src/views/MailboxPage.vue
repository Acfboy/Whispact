<template>
  <v-tabs v-model="tab" align-tabs="start" color="primary" density="compact">
    <v-tab :value="Tab.Inbox">收件箱</v-tab>
    <v-tab :value="Tab.OutBox">写信</v-tab>
  </v-tabs>

  <v-tabs-window v-model="tab">
    <v-tabs-window-item :value="Tab.Inbox">
      <v-card class="scroll-container">
        <v-card class="ma-4" v-for="[uuid, cover] in inboxCovers"
          v-bind:key="uuid" :prepend-icon="cover.sealed ? 'mdi-email-outline' : 'mdi-email-open-outline'" :subtitle="cover.timestamp"
          variant="outlined">
          <v-card-text>{{ cover.cover }}</v-card-text>
          <v-card-actions>
            <v-spacer></v-spacer>
            <v-btn v-if="cover.sealed" prepend-icon="mdi-email-open" @click="openMail(uuid)">拆封</v-btn>
            <v-btn v-else prepend-icon="mdi-file-search-outline" @click="readMail(uuid)">查看</v-btn>
          </v-card-actions>
        </v-card>
      </v-card>
    </v-tabs-window-item>
    <v-tabs-window-item :value="Tab.OutBox">
      <v-btn class="ml-4 mr-2 mt-2" variant="tonal" size="small" append-icon="mdi-plus" @click="edit(timeStampUuid())">
        新信件
      </v-btn>
      <v-card variant="text" class="scroll-container">
        <v-card class="ma-4" v-for="[uuid, cover] in draftsCovers" v-bind:key="uuid"
          :prepend-icon="cover.sealed ? 'mdi-email-outline' : 'mdi-email-open-outline'" :subtitle="cover.timestamp"
          variant="outlined">
          <v-card-text>{{ cover.cover }}</v-card-text>
          <v-card-actions>
            <v-spacer></v-spacer>
            <v-btn v-if="!cover.sealed" prepend-icon="mdi-email-open" @click="sealMail(uuid)" class="ma-1">漆封</v-btn>
            <v-btn v-else prepend-icon="mdi-email-fast-outline" class="ma-1" @click="send(uuid)">寄出</v-btn>
            <v-btn v-if="!cover.sealed" prepend-icon="mdi-email-edit-outline" class="ma-1"
              @click="edit(uuid)">内容</v-btn>
            <v-btn prepend-icon="mdi-image-edit-outline" class="ma-1" @click="editCover(uuid)">封面</v-btn>
            <v-btn prepend-icon="mdi-trash-can-outline" class="ma-1" @click="del(uuid)">删除</v-btn>
          </v-card-actions>
        </v-card>
      </v-card>
    </v-tabs-window-item>
  </v-tabs-window>

  <v-dialog v-model="deleteAlert">
    <v-card>
      <v-card-title>警告</v-card-title>
      <v-card-text>{{ `您真的要删除这封信吗？` }}</v-card-text>
      <template v-slot:actions>
        <v-btn @click="confirmDelete">我确定</v-btn>
        <v-btn @click="deleteAlert = false">点错了</v-btn>
      </template>
    </v-card>
  </v-dialog>

  <v-dialog v-model="enableCoverEditor">
    <v-card title="编辑封面">
      <v-card-text>
        <v-text-field label="编辑封面" variant="outlined" v-model="coverText"></v-text-field>
        <v-btn @click="saveCover" class="ma-1">保存</v-btn>
      </v-card-text>
    </v-card>
  </v-dialog>

  <touchPrompt v-model="touching" prompt="传递信件。"> </touchPrompt>

  <v-snackbar :timeout="2000" color="green-lighten-3" v-model="sentSuccess">
    已发送
  </v-snackbar>

</template>

<script setup lang="ts">
import { Mail, MailCoverList, MailInner, MessageType } from '@/types';
import { getTimeStamp, timeStampUuid, try_invoke } from '@/utils/utils';
import { emit, listen, UnlistenFn } from '@tauri-apps/api/event';
import { authenticate } from '@tauri-apps/plugin-biometric';
import { computed, ref, watchEffect } from 'vue';
import { onBeforeRouteLeave, useRouter } from 'vue-router';
import touchPrompt from '@/components/touch-prompt.vue';

enum Tab {
  Inbox = "Inbox",
  OutBox = "Outbox",
}

const lastTab = sessionStorage.getItem("mail-tab") || Tab.OutBox;
const tab = ref(lastTab as Tab);

const edit = async (uuid: string) => {
  if (!draftsCoverList.value?.mails.get(uuid)) {
    let data = new Map(draftsCoverList.value?.mails);
    data.set(uuid, { sealed: false, cover: "", timestamp: getTimeStamp() });
    draftsCoverList.value = { mails: data };
    await try_invoke("store_mail_drafts_covers", { data: draftsCoverList.value });
  }
  const mailData: MailInner = (await try_invoke("load_mail_inner", { uuid }))!;
  setTimeout(() => {
    router.push({ name: 'edit', params: { type: "Mail", id: uuid }, query: { title: mailData.title, body: mailData.body } });
  }, 100);
};

const deleteAlert = ref(false);
const toDeleteMail = ref("");
const del = (uuid: string) => {
  toDeleteMail.value = uuid;
  deleteAlert.value = true;
};
const confirmDelete = async () => {
  const uuid = toDeleteMail.value;
  let data = new Map(draftsCoverList.value?.mails);
  data.delete(uuid);
  draftsCoverList.value = { mails: data };
  await try_invoke("store_mail_drafts_covers", { data: draftsCoverList.value });
  await try_invoke("delete_mail", { uuid });
  deleteAlert.value = false;
};

const touching = ref(false);
const toSendMail = ref("");
const send = async (uuid: string) => {
  const inner: MailInner = (await try_invoke("load_mail_inner", { uuid }))!;
  const cover = draftsCoverList.value?.mails!.get(uuid);
  const data: Mail = { cover: cover!.cover, inner };
  await try_invoke("set_mail_msg", { mail: data });
  toSendMail.value = uuid;
  touching.value = true;
};

const enableCoverEditor = ref(false);
const coverText = ref("");
let coverToEdit: undefined | string = undefined;
const editCover = async (uuid: string) => {
  coverText.value = draftsCoverList.value!.mails.get(uuid)!.cover;
  coverToEdit = uuid;
  enableCoverEditor.value = true;
};
const saveCover = async () => {
  let data = new Map(draftsCoverList.value?.mails);
  if (coverToEdit) {
    let oriCover = data.get(coverToEdit);
    if (oriCover) {
      oriCover.cover = coverText.value;
      data.set(coverToEdit, oriCover);
      draftsCoverList.value = { mails: data };
      await try_invoke("store_mail_drafts_covers", { data: draftsCoverList.value });
    }
  }
  enableCoverEditor.value = false;
}

const sentSuccess = ref(false);
let unlisten: undefined | UnlistenFn = undefined;
(async () => {
  unlisten = await listen("touching", async (event: { payload: MessageType }) => {
    if (touching.value) {
      del(toSendMail.value);
      sentSuccess.value = true;
    }
    if (event.payload == MessageType.Mail) {
      const data: { mails: object } = (await try_invoke("load_mail_covers", {}))!;
      inboxCoverList.value = { mails: new Map(Object.entries(data.mails)) };
    }
  });
})();

watchEffect(async () => {
  if (touching.value == false)
    await try_invoke("clear_msg");
});

const openMail = async (uuid: string) => {
  const options = {
    allowDeviceCredential: true,
    title: '启封信件',
    subtitle: '进行验证后才可启封信件，启封后可查看正文。',
    confirmationRequired: true,
  };
  try {
    await authenticate('', options);
    let data = new Map(inboxCoverList.value?.mails);
    let cover = data.get(uuid);
    if (cover != undefined) {
      cover.sealed = false;
      data.set(uuid, cover);
      inboxCoverList.value = { mails: data };
      await try_invoke("store_mail_covers", { data: inboxCoverList.value });
    }
  } catch (err: unknown) {
    if (!(typeof err == 'object' && 'code' in err! && err.code == "userCancel"))
      emit("err", err);
  }
};

const sealMail = async (uuid: string) => {
  const options = {
    allowDeviceCredential: true,
    title: '漆封信件',
    subtitle: '漆封后无法修改其中内容，只能修改封面。',
    confirmationRequired: true,
  };
  try {
    await authenticate('', options);
    let data = new Map(draftsCoverList.value?.mails);
    let cover = data.get(uuid);
    if (cover != undefined) {
      cover.sealed = true;
      data.set(uuid, cover);
      draftsCoverList.value = { mails: data };
      await try_invoke("store_mail_drafts_covers", { data: draftsCoverList.value });
    }
  } catch (err: unknown) {
    if (!(typeof err == 'object' && 'code' in err! && err.code == "userCancel"))
      emit("err", err);
  }
};

const router = useRouter();

const readMail = async (uuid: string) => {
  const inner: MailInner = (await try_invoke("load_mail_inner", { uuid }))!;
  router.push({ name: "read", query: { title: inner.title, body: inner.body } });
};

const draftsCoverList = ref<MailCoverList>();
(async () => {
  const data: { mails: object } = (await try_invoke("load_mail_drafts_covers", {}))!;
  draftsCoverList.value = { mails: new Map(Object.entries(data.mails)) };
})();
const draftsCovers = computed(() => {
  let list = Array.from(draftsCoverList.value?.mails.entries() || []);
  list.sort();
  return list;
});

const inboxCoverList = ref<MailCoverList>();
(async () => {
  const data: { mails: object } = (await try_invoke("load_mail_covers", {}))!;
  inboxCoverList.value = { mails: new Map(Object.entries(data.mails)) };
})();
const inboxCovers = computed(() => {
  let list = Array.from(inboxCoverList.value?.mails.entries() || []);
  list.sort();
  return list;
});

watchEffect(async () => {
  if (inboxCoverList.value)
    await try_invoke("store_mail_covers", { data: inboxCoverList.value });
})


onBeforeRouteLeave(async () => {
  sessionStorage.setItem("mail-tab", tab.value);
  if (unlisten)
    unlisten();
})

</script>

<style scoped>
.scroll-container {
  overflow-y: auto;
  max-height: 75vh;
  padding-top: 0;
}
</style>