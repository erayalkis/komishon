import store from "../store";

const routes = [
  {
    path: "/",
    name: "Files",
    component: () => import("@/components/Home/Home.vue"),
    beforeEnter: (to) => {
      store.dispatch("loadInitialDirs");
      store.dispatch("setRoutePath", to);
    },
  },
  {
    path: "/start",
    name: "Welcome",
    component: () => import("@/components/Onboarding/Onboarding.vue"),
  },
  {
    path: "/favorites",
    name: "Favorites",
    component: () => import("@/components/Favorites/Favorites.vue"),
    beforeEnter: (to) => {
      store.dispatch("setRoutePath", to);
    },
  },
  {
    path: "/dev",
    name: "Debug",
    component: () => import("@/components/Dev.vue"),
  },
];

export default routes;
