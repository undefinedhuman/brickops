import { createRouter, createWebHistory } from "vue-router";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/",
      name: "home",
      component: () => import("@/views/HomeView.vue"),
      meta: {
        layout: "BaseLayout",
        requiresAuth: true
      }
    },
    {
      path: "/sets",
      name: "sets",
      component: () => import("@/views/SetsView.vue"),
      meta: {
        layout: "BaseLayout",
        requiresAuth: true
      }
    },
    {
      path: "/settings",
      name: "settings",
      component: () => import("@/views/SettingsView.vue"),
      meta: {
        layout: "BaseLayout",
        requiresAuth: true
      }
    }
  ]
});

export default router;
