import { defineStore } from "pinia";
import { ref } from "vue";

export const useThemeStore = defineStore("theme", () => {
  const value = ref("dark");

  function switchTheme() {
    value.value = value.value === "light" ? "dark" : "light";
  }

  return { value, switchTheme };
});
