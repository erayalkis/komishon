import { createApp } from "vue";
import "./style.css";
import App from "./App.vue";
import router from "./router/router";
import store from "./store/index";
import { appWindow, PhysicalSize } from "@tauri-apps/api/window";
import Calendar from "v-calendar/lib/components/calendar.umd";
import DatePicker from "v-calendar/lib/components/date-picker.umd";
import { setupCalendar } from "v-calendar";

appWindow.setMinSize(new PhysicalSize(1000, 600));
const app = createApp(App);

app.use(router);
app.use(store);
Vue.component("calendar", Calendar);
Vue.component("date-picker", DatePicker);

setupCalendar();

app.mount("#app");
