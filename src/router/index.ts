import { createRouter, createWebHashHistory } from "vue-router";
import HomePage from "../views/HomePage.vue";
import SealPage from "../views/SealPage.vue";
import SettingPage from "../views/SettingPage.vue";
import Disposable from "../components/disposable.vue";

const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    {
      path: "/",
      redirect: "/home",
    },
    {
      path: "/home",
      name: "Home",
      component: HomePage,
    },
    {
      path: "/settings",
      name: "Settings",
      component: SettingPage,
    },
    {
      path: "/seal",
      name: "Seal",
      component: SealPage,
    },
    {
      path: "/disposable",
      name: "disposable",
      component: Disposable,
    },
  ],
});
export default router;
