import { createApp } from "vue";
import { createPinia } from "pinia";
import "./assets/main.css";  // pastikan ada baris ini
import App from "./App.vue";
import router from "./router/index.js";

const app = createApp(App);
app.use(createPinia());
app.use(router);
app.mount("#app");
