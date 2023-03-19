import { invoke } from "@tauri-apps/api";

const notifications = {
  state: {
    notifications: [],
  },
  getters: {},
  mutations: {
    // Takes a `notification` parameter and adds it to the `state.notifications` array.
    addNotification(state, notification) {
      console.log(notification);
      state.notifications.push(notification);
    },
    // Takes a `notification` parameter and filters it out from the `state.notifications` array.
    removeNotification(state, notification) {
      state.notifications = state.notifications.filter(
        (notif) => notif.id != notification.id
      );
    },
  },
  actions: {
    // Invokes the get_notifications function to fetch all available notifications on the database as an array of notifications and returns it.
    async getNotifications(ctx) {
      return await invoke("get_notifications");
    },
    // Calls the `getNotifications` action and assigns the returned notifications array to the `state.notifications` property.
    async loadNotifications(ctx) {
      let notifs = await ctx.dispatch("getNotifications");
      ctx.state.notifications = notifs;
    },
    // Invokes the create_notification function to create a dummy notification.
    async createDebugNotif(ctx) {
      await invoke("create_notification", {
        title: "Debug Notif",
        body: "Debug Notif Body",
      });
    },
  },
};

export default notifications;
