import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";

export const useCategoryStore = defineStore("category", {
  state: () => ({
    categories: [],
    loading: false,
  }),

  actions: {
    async fetchAll() {
      this.loading = true;
      try {
        this.categories = await invoke("get_categories");
      } finally {
        this.loading = false;
      }
    },

    async add(name) {
      await invoke("add_category", { name });
      await this.fetchAll();
    },

    async remove(id) {
      await invoke("delete_category", { id });
      await this.fetchAll();
    },
  },
});
