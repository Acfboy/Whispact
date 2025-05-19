<template>
  <v-container class="fill-height">
    <v-container class="d-flex flex-column justify-center  align-center">
      <v-card @click="navigateTo('Disposable')" v-ripple prepend-icon="mdi-weather-windy" class="mb-8"
        variant="outlined" color="indigo" title="一次性消息" subtitle="这段话，只为这一刻" width="70vw">
        <v-card-text>
          文字仅现一次，读完永远消散。
        </v-card-text>
      </v-card>

      <v-card v-ripple prepend-icon="mdi-arrow-u-down-right" class="mb-8" variant="outlined" color="indigo"
        title="背靠背消息" subtitle="同步方位开启密语" width="70vw">
        <v-card-text>
          触碰后转身背对背，才会显示消息。
        </v-card-text>
      </v-card>

      <v-card v-ripple prepend-icon="mdi-clock-check-outline" class="mb-8" variant="outlined" color="indigo"
        title="打卡此刻" subtitle="触碰封存时光胶囊" width="70vw">
        <v-card-text>
          共同记下这值得纪念的一刻。
        </v-card-text>
      </v-card>
    </v-container>
  </v-container>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router';
const router = useRouter();
const navigateTo = (routeName: string) => {
  setTimeout(() => {
    router.push({ name: routeName });
  }, 100);
};
</script>