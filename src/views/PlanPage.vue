<template>
  <v-tabs v-model="tab" align-tabs="start" color="primary" density="compact">
    <v-tab :value="Tab.Finished">已完成</v-tab>
    <v-tab :value="Tab.Plan">未完成</v-tab>
  </v-tabs>

  <v-tabs-window v-model="tab">
    <v-tabs-window-item :value="Tab.Finished">
      <v-list class="ma-2" lines="two">
        <v-list-item v-for="(c, i) in totalFinished.list" :ket="i">
          <v-list-item-title>{{ c.plan.title }}</v-list-item-title>
          <v-list-item-subtitle>{{ c.plan.body }}</v-list-item-subtitle>
          <template v-slot:append> <v-list-item-action>
              <small class="text-high-emphasis opacity-60">
                {{ c.time }}
              </small>
            </v-list-item-action>
          </template>
        </v-list-item>
      </v-list>
    </v-tabs-window-item>

    <v-tabs-window-item :value="Tab.Plan">
      <v-list class="ma-2" lines="two">
        <v-list-item v-for="[uuid, plan] in planDrafts.drafts" :active="false" color="primary" :key="uuid">
          <v-list-item-title>{{ plan.title }}</v-list-item-title>
          <v-list-item-subtitle>{{ plan.body }}</v-list-item-subtitle>
          <template v-slot:append>
            <v-btn icon="mdi-pencil" variant="text" @click="edit(uuid)"></v-btn>
            <v-btn icon="mdi-trash-can-outline" variant="text" @click="del(uuid)"></v-btn>
          </template>
        </v-list-item>
      </v-list>
      <v-btn class="ma-4" @click="edit(randomUUID())">
        新计划
      </v-btn>
      <v-btn class="ma-4" @click="sync()">
        同步计划
      </v-btn>
    </v-tabs-window-item>
  </v-tabs-window>

  <v-dialog v-model="deleteAlert">
    <v-card>
      <v-card-title>警告</v-card-title>
      <v-card-text>{{ `您真的要删除${planDrafts.drafts.get(toDeleteUuid)?.title}吗？` }}</v-card-text>
      <template v-slot:actions>
        <v-btn @click="confirmDelete">我确定</v-btn>
        <v-btn @click="deleteAlert = false">点错了</v-btn>
      </template>
    </v-card>
  </v-dialog>

  <touch-prompt v-model="touching" prompt="记录计划完成。"> </touch-prompt>

  <v-snackbar :timeout="2000" color="red-lighten-1" v-model="conflictAlert">
    {{ `同步失败：双方的计划有冲突。(第一个冲突：${conflictPrompt})` }}
  </v-snackbar>
  <v-snackbar :timeout="2000" color="green-lighten-3" v-model="syncSuccess">
    计划同步成功
  </v-snackbar>

</template>

<script setup lang="ts">
import { FinishedPlanList, Plan, PlanDrafts, SealedInstances, SyncPlans } from '@/types';
import { invoke } from '@tauri-apps/api/core';
import { randomUUID } from '@/utils/utils';
import { computed, onMounted, ref } from 'vue';
import { onBeforeRouteLeave, useRouter } from 'vue-router';
import touchPrompt from '@/components/touch-prompt.vue';
import { listen, UnlistenFn } from '@tauri-apps/api/event';

enum Tab {
  Finished = "Finished",
  Plan = "Plan",
}

const lastTab = sessionStorage.getItem("tab") || Tab.Plan;
const tab = ref(lastTab as Tab);

const deleteAlert = ref(false);
const toDeleteUuid = ref("");
const router = useRouter();
const edit = (uuid: string) => {
  const plan = planDrafts.value.drafts.get(uuid);
  setTimeout(() => {
    router.push({ name: 'edit', params: { type: "Plan", id: uuid }, query: { title: plan?.title, body: plan?.body } });
  }, 100);
};
const del = (uuid: string) => {
  toDeleteUuid.value = uuid;
  deleteAlert.value = true;
};
const confirmDelete = async () => {
  let deleted: PlanDrafts = { drafts: new Map(planDrafts.value.drafts) };
  deleted.drafts.delete(toDeleteUuid.value);
  await invoke("store_plan_drafts", { data: deleted });
  deleteAlert.value = false;
  planDrafts.value = deleted;
};

const planDrafts = ref<PlanDrafts>({ drafts: new Map() });
const sealed = ref<SealedInstances>({ instances: [] });
const finishedPlans = ref<FinishedPlanList>({ list: [] });

const touching = ref(false);
const touchingPrompt = ref("");
const planToFinish = ref<undefined | string>(undefined);
const sync = async () => {
  const data: SyncPlans = {
    selectedPlan: planToFinish.value,
    plans: planDrafts.value.drafts,
  }
  await invoke("set_plan_sync_msg", { data });
  touching.value = true;
};

const conflictAlert = ref(false);
const conflictPrompt = ref("");

let unlisenData: undefined | UnlistenFn = undefined;
const syncSuccess = ref(false);
(async () => {
  unlisenData = await listen<{ selectedPlans?: string, plans: {} }>("recv-plan-sync", async (event) => {
    const counterPlan: Map<string, Plan> = new Map(Object.entries(event.payload.plans));
    const conflict = [...counterPlan.entries()].some(([uuid, plan]) => {
      const entry = planDrafts.value.drafts.get(uuid);
      if (entry && (entry.body != plan.body || entry.title != plan.title)) {
        conflictPrompt.value = entry.title;
        return true;
      } else {
        return false;
      }
    });
    if (conflict) {
      conflictAlert.value = true;
      return;
    }

    let newData = new Map(planDrafts.value.drafts);
    counterPlan.forEach((plan: Plan, uuid: string) => {
      newData.set(uuid, plan);
    });
    planDrafts.value = { drafts: newData };
    await invoke('store_plan_drafts', { data: planDrafts.value });

    syncSuccess.value = true;              
  });
})();

const totalFinished = computed(() => {
  (async () => {
    const data: SyncPlans = {
      plans: planDrafts.value.drafts
    }
    await invoke("set_plan_sync_msg", { data });
  })();

  return {
    list: [
      ...finishedPlans.value.list,
      ...sealed.value.instances.map(({ instance, time }) => ({
        plan: {
          title: instance,
          body: ''
        },
        time
      }))
    ].sort((a, b) => new Date(a.time).getTime() - new Date(b.time).getTime())
  };
});


onMounted(async () => {
  sealed.value = await invoke("load_sealed_instances");
  const loaded = await invoke<PlanDrafts>("load_plan_drafts");
  planDrafts.value = { drafts: new Map(Object.entries(loaded.drafts || {})) };
  finishedPlans.value = await invoke("load_finished_plan_list");
});

onBeforeRouteLeave(() => {
  sessionStorage.setItem("tab", tab.value);
  if (unlisenData != undefined) {
    unlisenData();
  }
})

</script>