import { createRouter, createWebHashHistory, RouteRecordRaw } from "vue-router";

const rutes: RouteRecordRaw[] = [
  {
    path: "/",
    component: () => import("./pages/Initial.vue"),
  },
  {
    path: "/config",
    component: () => import("./pages/Filter.vue"),
  },
  {
    path: "/bs-upload",
    component: () => import("./pages/BsUpload.vue"),
  },
];

export const routes = createRouter({
  history: createWebHashHistory(),
  routes: rutes
})