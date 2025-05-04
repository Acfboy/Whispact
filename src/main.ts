import { createApp } from "vue";
import App from "./App.vue";

const app = createApp(App);

app.config.errorHandler = (err) => {
  if (err instanceof Error) {
    alert("错误：" + err.message);
  }
};

app.mount("#app");
