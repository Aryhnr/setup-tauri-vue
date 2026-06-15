import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";
import { useToast } from "../composables/useToast";

const STATUS_LABEL = {
  ANTRIAN: "Antrian",
  DIPROSES: "Diproses",
  SELESAI: "Selesai",
  DIBAYAR: "Dibayar",
};

export const usePrintOrderStore = defineStore("printOrder", {
  state: () => ({
    orders: [],
    loading: false,
    error: null,
  }),

  actions: {
    async fetchAll(status = null) {
      this.loading = true;
      this.error = null;
      try {
        this.orders = await invoke("get_print_orders", { status });
      } catch (err) {
        this.error = err?.toString() ?? "Gagal memuat order percetakan";
        console.error(err);
      } finally {
        this.loading = false;
      }
    },

    async add(payload) {
      const toast = useToast();
      const order = {
        id: null,
        order_no: null,
        service_type: payload.service_type,
        paper_size: payload.paper_size || null,
        color_mode: payload.color_mode || null,
        pages: Number(payload.pages) || 1,
        copies: Number(payload.copies) || 1,
        price_per_page: Number(payload.price_per_page) || 0,
        total_price: Number(payload.total_price) || 0,
        status: "ANTRIAN",
        created_at: null,
      };
      await invoke("add_print_order", { order });
      await this.fetchAll();
      toast.success("Order percetakan berhasil ditambahkan");
    },

    async update(id, payload) {
      const toast = useToast();
      const order = {
        id,
        order_no: null,
        service_type: payload.service_type,
        paper_size: payload.paper_size || null,
        color_mode: payload.color_mode || null,
        pages: Number(payload.pages) || 1,
        copies: Number(payload.copies) || 1,
        price_per_page: Number(payload.price_per_page) || 0,
        total_price: Number(payload.total_price) || 0,
        status: payload.status,
        created_at: null,
      };
      await invoke("update_print_order", { id, order });
      await this.fetchAll();
      toast.success("Order percetakan berhasil diperbarui");
    },

    async updateStatus(id, status) {
      const toast = useToast();
      await invoke("update_print_order_status", { id, status });
      await this.fetchAll();
      toast.info(`Status order diubah ke: ${STATUS_LABEL[status] ?? status}`);
    },

    async remove(id) {
      const toast = useToast();
      await invoke("delete_print_order", { id });
      await this.fetchAll();
      toast.success("Order berhasil dihapus");
    },
  },
});
