import { invoke } from "@tauri-apps/api";

const notifications = {
  state: {
    notifications: [],
  },
  getters: {},
  mutations: {
    addNotification(state, notification) {
      console.log(notification);
      state.notifications.push(notification);
    },
    removeNotification(state, notification) {
      state.notifications = state.notifications.filter(
        (notif) => notif.id != notification.id
      );
    },
  },
  actions: {
    async getNotifications(ctx) {
      let notifs = await invoke("get_notifications");
      console.log(notifs);
      return notifs;
    },
    async createDebugNotif(ctx) {
      await invoke("create_notification", {
        title: "Debug Notif",
        body: "Debug Notif Body",
      });
    },
  },
};

export default notifications;
