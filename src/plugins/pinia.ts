import { createPinia } from "pinia";
import { markRaw } from "vue";
import router from "../router";

export default createPinia().use(({ store }) => {
  store.router = markRaw(router);
});
