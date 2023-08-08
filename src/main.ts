import { createApp } from "vue";

// Main
import App from "./App.vue";

// Oh Vue Icons
import { OhVueIcon, addIcons } from "oh-vue-icons";
import { BiMoonStarsFill } from "oh-vue-icons/icons";

// CSS - Reset Scrollbar
import "./assets/reset.css";
import "./assets/tailwind.css";

addIcons(BiMoonStarsFill);

// Router
import router from "./router/index";
import BaseLayout from "./layouts/BaseLayout.vue";

// Plugins
import pinia from "./plugins/pinia";
import { api } from "./plugins/api";

createApp(App)
  .component("v-oh-icon", OhVueIcon)
  .component("BaseLayout", BaseLayout)
  .use(router)
  .use(pinia)
  .use(api)
  .mount("#app");
