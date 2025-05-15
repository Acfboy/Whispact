import { createApp } from "vue";
import 'vuetify/styles'
import { createVuetify } from 'vuetify'
import App from './App.vue'
import { components, directives } from "vuetify/dist/vuetify.js";

const vuetify = createVuetify({
  components,
  directives,
})

const app = createApp(App);

app.config.errorHandler = (err) => {
  if (err instanceof Error) {
    alert("错误：" + err.message);
  }
};

createApp(App).use(vuetify).mount('#app')