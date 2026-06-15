import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";
import { useToast } from "../composables/useToast";

export const useSupplierStore = defineStore("supplier", {
  state: () => ({
    suppliers: [],
    loading: false,
    error: null,
  }),

  actions: {
    async fetchAll(keyword = "") {
      this.loading = true;
      this.error = null;
      try {
        this.suppliers = await invoke("get_suppliers", {
          keyword: keyword || null,
        });
      } catch (err) {
        this.error = err?.toString() ?? "Gagal memuat data supplier";
        console.error(err);
      } finally {
        this.loading = false;
      }
    },

    async add(payload) {
      const toast = useToast();
      const supplier = {
        id: null,
        name: payload.name,
        phone: payload.phone || null,
        address: payload.address || null,
        notes: payload.notes || null,
      };
      await invoke("add_supplier", { supplier });
      await this.fetchAll();
      toast.success(`Supplier "${payload.name}" berhasil ditambahkan`);
    },

    async update(id, payload) {
      const toast = useToast();
      const supplier = {
        id,
        name: payload.name,
        phone: payload.phone || null,
        address: payload.address || null,
        notes: payload.notes || null,
      };
      await invoke("update_supplier", { id, supplier });
      await this.fetchAll();
      toast.success(`Supplier "${payload.name}" berhasil diperbarui`);
    },

    async remove(id) {
      const toast = useToast();
      const name = this.suppliers.find((s) => s.id === id)?.name ?? "Supplier";
      await invoke("delete_supplier", { id });
      await this.fetchAll();
      toast.success(`"${name}" berhasil dihapus`);
    },
  },
});
