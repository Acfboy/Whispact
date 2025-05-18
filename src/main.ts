import { createApp } from "vue";
import "vuetify/styles";
import "@mdi/font/css/materialdesignicons.css"; 
import { createVuetify } from "vuetify";
import App from "./App.vue";
import { components, directives } from "vuetify/dist/vuetify.js";
import router from "./router";
import { error } from "@tauri-apps/plugin-log";

const vuetify = createVuetify({
  components,
  directives,
});

const app = createApp(App);

app.use(vuetify);
app.use(router);

app.config.errorHandler = (err) => {
  if (err instanceof Error) {
    alert("错误：" + err.message);
    error(err.message);
  }
};

app.mount("#app");
