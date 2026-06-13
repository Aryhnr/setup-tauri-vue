import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";

export const useServiceStore = defineStore("service", {
  state: () => ({
    services: [],
    summaryToday: { total_sell: 0, total_profit: 0, count: 0 },
    loading: false,
    error: null,
  }),

  actions: {
    /**
     * Ambil seluruh transaksi jasa digital, opsional filter service_type.
     * @param {string|null} serviceType - TOKEN_PLN / TOPUP / PULSA / PAKET_DATA
     */
    async fetchAll(serviceType = null) {
      this.loading = true;
      this.error = null;
      try {
        this.services = await invoke("get_services", { serviceType });
      } catch (err) {
        this.error = err?.toString() ?? "Gagal memuat data jasa";
        console.error(err);
      } finally {
        this.loading = false;
      }
    },

    /**
     * Tambah transaksi jasa digital baru. Profit dihitung otomatis di backend.
     * @param {Object} payload
     */
    async add(payload) {
      const sellPrice = Number(payload.sell_price) || 0;
      const buyPrice = Number(payload.buy_price) || 0;
      const service = {
        id: null,
        service_type: payload.service_type,
        description: payload.description || null,
        customer_info: payload.customer_info || null,
        sell_price: sellPrice,
        buy_price: buyPrice,
        profit: sellPrice - buyPrice,
        transaction_date: null,
      };
      await invoke("add_service", { service });
      await this.fetchAll();
      await this.fetchSummaryToday();
    },

    /**
     * Hapus transaksi jasa digital.
     * @param {number} id
     */
    async remove(id) {
      await invoke("delete_service", { id });
      await this.fetchAll();
      await this.fetchSummaryToday();
    },

    /**
     * Ambil ringkasan total penjualan & profit jasa hari ini.
     */
    async fetchSummaryToday() {
      try {
        this.summaryToday = await invoke("get_service_summary_today");
      } catch (err) {
        console.error(err);
      }
    },
  },
});
