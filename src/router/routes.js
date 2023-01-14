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
    path: "/deadlines",
    name: "Deadlines",
    component: () => import("@/components/Deadlines/Deadlines.vue"),
    beforEnter: (to) => {
      store.dispatch("setRoutePath", to);
    },
  },
];

export default routes;
