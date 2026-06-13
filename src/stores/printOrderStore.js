import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";

export const usePrintOrderStore = defineStore("printOrder", {
  state: () => ({
    orders: [],
    loading: false,
    error: null,
  }),

  actions: {
    /**
     * Ambil seluruh order percetakan, opsional filter status.
     * @param {string|null} status - ANTRIAN / DIPROSES / SELESAI / DIBAYAR
     */
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

    /**
     * Tambah order percetakan baru. order_no di-generate otomatis backend.
     * @param {Object} payload
     */
    async add(payload) {
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
    },

    /**
     * Update detail order (tidak mengubah status).
     * @param {number} id
     * @param {Object} payload
     */
    async update(id, payload) {
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
    },

    /**
     * Update status order: ANTRIAN -> DIPROSES -> SELESAI -> DIBAYAR.
     * @param {number} id
     * @param {string} status
     */
    async updateStatus(id, status) {
      await invoke("update_print_order_status", { id, status });
      await this.fetchAll();
    },

    /**
     * Hapus order.
     * @param {number} id
     */
    async remove(id) {
      await invoke("delete_print_order", { id });
      await this.fetchAll();
    },
  },
});
