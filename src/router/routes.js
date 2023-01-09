const routes = [
  {
    path: "/",
    name: "Home",
    component: () => import("@/components/Home/Home.vue"),
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
  },
  {
    path: "/dev",
    name: "Debug",
    component: () => import("@/components/Dev.vue"),
  },
];

export default routes;
