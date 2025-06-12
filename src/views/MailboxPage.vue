<template>
  <v-tabs v-model="tab" align-tabs="start" color="primary" density="compact">
    <v-tab :value="Tab.Inbox">收件箱</v-tab>
    <v-tab :value="Tab.OutBox">写信</v-tab>
  </v-tabs>

  <v-tabs-window v-model="tab">
    <v-tabs-window-item :value="Tab.Inbox">
      <v-card class="scroll-container">
        <v-card class="ma-4" v-for="[uuid, cover] in inboxCovers"
          :prepend-icon="cover.sealed ? 'mdi-email-outline' : 'mdi-email-open-outline'" :subtitle="cover.timestamp"
          variant="outlined">
          <v-card-text>{{ cover.cover }}</v-card-text>
          <v-card-actions>
            <v-spacer></v-spacer>
            <v-btn v-if="cover.sealed" prepend-icon="mdi-email-open" @click="openMail(uuid)">拆封</v-btn>
            <v-btn v-else prepend-icon="mdi-file-search-outline">查看</v-btn>
          </v-card-actions>
        </v-card>
      </v-card>
    </v-tabs-window-item>
    <v-tabs-window-item :value="Tab.OutBox">
      <!-- <v-btn class="ml-4 mr-2 mt-2" variant="tonal" size="small" append-icon="mdi-plus" @click="edit(timeStampUuid())">
        新信件
      </v-btn>
      <v-card variant="text" class="scroll-container">
        <v-card class="ma-4" v-for="[uuid, cover] in inboxCovers"
          :prepend-icon="cover.sealed ? 'mdi-email-outline' : 'mdi-email-open-outline'" :subtitle="cover.timestamp"
          variant="outlined">
          <v-card-text>{{ cover.cover }}</v-card-text>
          <v-card-actions>
            <v-spacer></v-spacer>
            <v-btn v-if="cover.sealed" prepend-icon="mdi-email-open" @click="openMail(uuid)">拆封</v-btn>
            <v-btn v-else prepend-icon="mdi-file-search-outline">查看</v-btn>
          </v-card-actions>
        </v-card>
      </v-card> -->
    </v-tabs-window-item>
  </v-tabs-window>

  <!-- <v-dialog v-model="deleteAlert">
  <v-card>
    <v-card-title>警告</v-card-title>
    <v-card-text>{{ `您真的要删除${toDeletePlanTitle}吗？` }}</v-card-text>
    <template v-slot:actions>
        <v-btn @click="confirmDelete">我确定</v-btn>
        <v-btn @click="deleteAlert = false">点错了</v-btn>
      </template>
</v-card>
</v-dialog>

<touch-prompt v-model="touching" :prompt="touchingPrompt"> </touch-prompt>

<v-snackbar :timeout="2000" color="green-lighten-3" v-model="sentSuccess">
  已发送
</v-snackbar> -->

</template>

<script setup lang="ts">
// import { FinishedPlanList, Plan, PlanDrafts, SealedInstances, SyncPlans } from '@/types';
// import { getTimeStamp, timeStampUuid, try_invoke } from '@/utils/utils';
import { MailCoverList } from '@/types';
import { try_invoke } from '@/utils/utils';
import { emit } from '@tauri-apps/api/event';
import { authenticate } from '@tauri-apps/plugin-biometric';
import { computed, ref, watchEffect } from 'vue';
import { onBeforeRouteLeave } from 'vue-router';
// import { onBeforeRouteLeave, useRouter } from 'vue-router';
// import touchPrompt from '@/components/touch-prompt.vue';
// import { listen, UnlistenFn } from '@tauri-apps/api/event';

enum Tab {
  Inbox = "Inbox",
  OutBox = "Outbox",
}

const lastTab = sessionStorage.getItem("mail-tab") || Tab.OutBox;
const tab = ref(lastTab as Tab);

// const deleteAlert = ref(false);
// const toDeleteUuid = ref("");
// const router = useRouter();
// const edit = (uuid: string) => {
//   const plan = planDrafts.value.drafts.get(uuid);
//   setTimeout(() => {
//     router.push({ name: 'edit', params: { type: "Plan", id: uuid }, query: { title: plan?.title, body: plan?.body } });
//   }, 100);
// };

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
    if (cover) {
      cover.sealed = false;
      data.set(uuid, cover);
      inboxCoverList.value = { mails: data };
    }
  } catch (err: unknown) {
    if (!(typeof err == 'object' && 'code' in err! && err.code == "userCancel"))
      emit("err", err);
  }
};

const draftsCoverList = ref<MailCoverList>();
(async () => {
  const data: { mails: object } = (await try_invoke("load_mail_drafts_covers", {}))!;
  draftsCoverList.value = { mails: new Map(Object.entries(data.mails)) };
})();

const inboxCoverList = ref<MailCoverList>();
(async () => {
  const data: { mails: object } = (await try_invoke("load_mail_covers", {}))!;
  inboxCoverList.value = { mails: new Map(Object.entries(data.mails)) };
})();
const inboxCovers = computed(() => {
  return Array.from(inboxCoverList.value?.mails.entries() || []);
});

watchEffect(async () => {
  if (inboxCoverList.value)
    await try_invoke("store_mail_covers", { data: inboxCoverList.value });
})


onBeforeRouteLeave(() => {
  sessionStorage.setItem("mail-tab", tab.value);
})

</script>

<style scoped>
.scroll-container {
  overflow-y: auto;
  max-height: 75vh;
  padding-top: 0;
}
</style>