import { createRouter, createWebHashHistory } from "vue-router";
import HomePage from "../views/HomePage.vue";
import SealPage from "../views/SealPage.vue";
import SettingPage from "../views/SettingPage.vue";
import Disposable from "../components/disposable.vue";
import LogPage from "../views/LogPage.vue";

const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    {
      path: "/",
      redirect: "/home",
    },
    {
      path: "/home",
      name: "home",
      component: HomePage,
    },
    {
      path: "/settings",
      name: "settings",
      component: SettingPage,
    },
    {
      path: "/seal",
      name: "seal",
      component: SealPage,
    },
    {
      path: "/disposable",
      name: "Disposable",
      component: Disposable,
    },
    {
      path: "/log",
      name: "log",
      component: LogPage,
    },
  ],
});
export default router;
