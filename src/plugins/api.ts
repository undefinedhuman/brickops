import type { InjectionKey, Plugin } from "vue";
import type { AxiosInstance } from "axios";
import axios from "axios";

export const $api: InjectionKey<AxiosInstance> = Symbol("$api");

export const api: Plugin = {
  install(app) {
    app.provide(
      $api,
      axios.create({
        baseURL: `${import.meta.env.VITE_APP_API_URL}`
      })
    );
  }
};
