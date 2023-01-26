import { createApp } from "vue";
import "./style.css";
import App from "./App.vue";
import router from "./router/router";
import store from "./store/index";
import { appWindow, PhysicalSize } from "@tauri-apps/api/window";
import VCalendar from "v-calendar";

appWindow.setMinSize(new PhysicalSize(1000, 600));
const app = createApp(App);

app.use(router);
app.use(store);
app.use(VCalendar, {});

app.mount("#app");
