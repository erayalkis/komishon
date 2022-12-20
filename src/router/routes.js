const routes = [
  {
    path: "/",
    name: "Home",
    component: () => import("@/components/Home.vue"),
  },
  {
    path: "/start",
    name: "Welcome",
    component: () => import("@/components/Onboarding.vue"),
  },
  {
    path: "/files",
    name: "Files",
    component: () => import("@/components/Files.vue"),
  },
  {
    path: "/dev",
    name: "Debug",
    component: () => import("@/components/Dev.vue"),
  },
];

export default routes;
