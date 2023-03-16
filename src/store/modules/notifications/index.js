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
      return await invoke("get_notifications");
    },
    async loadNotifications(ctx) {
      let notifs = await ctx.dispatch("getNotifications");
      ctx.state.notifications = notifs;
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
