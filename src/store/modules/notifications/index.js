import { invoke } from "@tauri-apps/api";

const modals = {
  state: {
    notifications: [],
  },
  getters: {},
  mutations: {
    addNotification(state, notification) {
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
  },
};

export default modals;
