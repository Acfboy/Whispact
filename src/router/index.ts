import {  createRouter, createWebHashHistory } from "vue-router"
import MapComponent from '../components/hello.vue'  
 
const router = createRouter({
  history: createWebHashHistory(), 
  routes: [
    {
      path: '/',
      redirect: '/home'
    },
    {
      path: '/home', 
      name: 'Home', 
      component: MapComponent 
    },
    {
      path: '/settings', 
      name: 'Settings', 
      component: MapComponent 
    },
    {
      path: '/seal', 
      name: 'Seal', 
      component: MapComponent 
    },
  ]
})
export default router