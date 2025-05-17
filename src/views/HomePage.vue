<template>
  <v-container class="fill-height">
    <v-row justify="center" align="center">
      <v-col cols="12" sm="8" md="6">
        <v-card class="mx-auto">
          <v-card-title class="text-center text-h5 font-weight-bold">
            主要功能
          </v-card-title>
          <v-card-text>
            <v-row justify="center" class="mt-4">
              <v-col cols="12" class="text-center">
                <v-btn
                  color="primary"
                  size="large"
                  class="ma-2"
                  prepend-icon="mdi-swap-horizontal"
                  to="/transfer"
                >
                  背对背传递
                </v-btn>
              </v-col>
              <v-col cols="12" class="text-center">
                <v-btn
                  color="success"
                  size="large"
                  class="ma-2"
                  prepend-icon="mdi-calendar-check"
                  to="/checkin"
                >
                  打卡
                </v-btn>
              </v-col>
              <v-col cols="12" class="text-center">
                <v-btn
                  color="info"
                  size="large"
                  class="ma-2"
                  prepend-icon="mdi-message-text"
                  to="/disposable"
                >
                  一次性消息
                </v-btn>
              </v-col>
            </v-row>
          </v-card-text>
        </v-card>
      </v-col>
    </v-row>
  </v-container>
</template>

<script setup lang="ts">
// 组件逻辑可以在这里添加
</script>

<style scoped>
.v-btn {
  width: 200px;
}
</style>