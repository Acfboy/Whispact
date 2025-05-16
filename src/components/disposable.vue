<template>
    <v-container fluid>
      <v-input
        v-model="message"
        placeholder="请输入消息..."
        append-icon="mdi-send"
        :append-loading="isSending"
        @click:append="handleSend"
        class="mt-4"
        >
      </v-input>
    </v-container>
  </template>
  
  <script setup>
  // import { ref } from 'vue';
  
  // // 输入内容
  // const message = ref('');
  // // 发送状态（防止重复点击）
  // const isSending = ref(false);
  
  // const handleSend = async () => {
  //   if (!message.value.trim()) return; // 防止空消息发送
    
  //   try {
  //     isSending.value = true; // 显示加载状态
  //     // 模拟发送请求（替换为实际 API 调用）
  //     await new Promise(resolve => setTimeout(resolve, 1000));
  //     console.log('发送内容:', message.value);
  //     message.value = ''; // 清空输入框
  //   } finally {
  //     isSending.value = false; // 结束加载状态
  //   }
  // };
  </script>
  
  <style scoped>
  /* 自定义样式：调整输入框高度、边框样式 */
  .v-input {
    max-width: 800px;
    margin: 0 auto;
  }
  
  .v-textarea {
    min-height: 80px;
    padding: 12px 16px;
    border: 1px solid #e0e0e0 !important;
    border-radius: 24px;
    box-shadow: none !important;
  }
  
  /* 发送按钮样式 */
  .v-input__append-inner {
    padding: 8px;
  }
  
  .mdi-send {
    font-size: 20px;
    color: #42a5f5; /* 蓝色主题色 */
  }
  </style>