import { createApp } from "vue";
import 'vuetify/styles'
import '@mdi/font/css/materialdesignicons.css' // 导入图标库
import { createVuetify } from 'vuetify'
import App from './App.vue'
import { components, directives } from "vuetify/dist/vuetify.js";
import router from './router'
import Map from './components/hello.vue' 

const vuetify = createVuetify({
  components,
  directives,
})

const app = createApp(App);

app.use(vuetify)
app.use(router)
app.component('Map', Map)

app.config.errorHandler = (err) => {
  if (err instanceof Error) {
    alert("错误：" + err.message);
  }
};

app.mount('#app')