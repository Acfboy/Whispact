<template>
  <v-tabs v-model="tab" align-tabs="start" color="primary" density="compact">
    <v-tab :value="Tab.Finished">已完成</v-tab>
    <v-tab :value="Tab.Plan">未完成</v-tab>
  </v-tabs>

  <v-tabs-window v-model="tab">
    <v-tabs-window-item :value="Tab.Finished">
      <v-timeline align="start">
        <v-timeline-item v-for="(c, i) in totalFinished.list" :key="i" size="small">
          <template v-slot:opposite>
            <div class="pt-1" v-text="c.time"></div>
          </template>
          <div>
            <v-expansion-panel :title="c.plan.title" :text="c.plan.body">
            </v-expansion-panel>
          </div>
        </v-timeline-item>
      </v-timeline>
    </v-tabs-window-item>

    <v-tabs-window-item :value="Tab.Plan">
      Two
    </v-tabs-window-item> 
  </v-tabs-window>
</template>

<script setup lang="ts">
import { FinishedPlanList, PlanDrafts, SealedInstances } from '@/types';
import { invoke } from '@tauri-apps/api/core';
import { onMounted, ref } from 'vue';

enum Tab {
  Finished,
  Plan,
}

const tab = ref(Tab.Finished);

const sealed = ref<SealedInstances>({ instances: [] });
const planDrafts = ref<PlanDrafts>();
const finishedPlans = ref<FinishedPlanList>({ list: [] });
const totalFinished = ref<FinishedPlanList>({ list: [] });


onMounted(async () => {
  sealed.value = await invoke("load_sealed_instances");
  planDrafts.value = await invoke("load_plan_drafts");
  finishedPlans.value = await invoke("load_finished_plan_list");
  totalFinished.value = {
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

</script>