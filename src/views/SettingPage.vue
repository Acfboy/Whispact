<template>
  <v-container>
    <v-card variant="flat" title="开发人员选项">
      <v-card-text>
        <v-btn variant="outlined" @click="testCommnication" class="ma-1">
          测试通信
        </v-btn>
        <v-btn variant="outlined" @click="navigateTo('log')" class="ma-1">
          查看日志
        </v-btn>
        <v-btn variant="outlined" @click="genRandomSeal" class="ma-1">
          添加随机打卡
        </v-btn>
        <v-btn variant="outlined" @click="genRandomPlan" class="ma-1">
          添加未完成计划
        </v-btn>
      </v-card-text>
    </v-card>
  </v-container>
</template>

<script setup lang="ts">
import { useRouter } from "vue-router";
import { testCommnication, genRandomSeal, genRandomPlan } from "@/utils/utils";

const router = useRouter();

const navigateTo = (routerName: string) => {
  router.push({ name: routerName });
}
</script>