const routes = [
  {
    path: "/",
    name: "Home",
    component: () => import("@/components/Home.vue"),
  },
  {
    path: "/files",
    name: "Files",
    component: () => import("@/components/Files.vue"),
  },
];

export default routes;
