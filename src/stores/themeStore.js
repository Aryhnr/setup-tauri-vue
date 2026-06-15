import { defineStore } from "pinia";

const THEME_KEY = "toko_theme";

export const useThemeStore = defineStore("theme", {
  state: () => ({
    isDark: false,
  }),

  actions: {
    /** Inisialisasi tema dari localStorage — dipanggil sekali saat app start. */
    init() {
      const saved = localStorage.getItem(THEME_KEY);
      // Jika belum pernah set, ikuti preferensi sistem
      if (saved === null) {
        this.isDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
      } else {
        this.isDark = saved === "dark";
      }
      this._applyTheme();
    },

    toggle() {
      this.isDark = !this.isDark;
      localStorage.setItem(THEME_KEY, this.isDark ? "dark" : "light");
      this._applyTheme();
    },

    _applyTheme() {
      document.documentElement.classList.toggle("dark", this.isDark);
    },
  },
});
