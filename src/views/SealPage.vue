<template>
  <v-container class="fill-height justify-center align-items ">
    <v-text-field variant="solo-filled" hint="输入要打卡的此刻，点击按钮后相碰确认。" append-inner-icon="mdi-send"
      @click:append-inner="onClick" persistent-hint v-model="msg"></v-text-field>

    <v-dialog v-model="syncTouch" width="auto"><v-icon size="30vw">
        mdi-access-point
      </v-icon></v-dialog>


  </v-container>
</template>

<script setup="ts">
import { ref } from "vue";

const msg = ref("");
const syncTouch = ref(false);

const onClick = () => {
  syncTouch.value = 1;
}
</script>