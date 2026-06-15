import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";
import { useToast } from "../composables/useToast";

export const useServiceStore = defineStore("service", {
  state: () => ({
    services: [],
    summaryToday: { total_sell: 0, total_profit: 0, count: 0 },
    loading: false,
    error: null,
  }),

  actions: {
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

    async add(payload) {
      const toast = useToast();
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
      toast.success(
        `Jasa "${payload.description || payload.service_type}" berhasil dicatat`,
      );
    },

    async remove(id) {
      const toast = useToast();
      await invoke("delete_service", { id });
      await this.fetchAll();
      await this.fetchSummaryToday();
      toast.success("Transaksi jasa berhasil dihapus");
    },

    async fetchSummaryToday() {
      try {
        this.summaryToday = await invoke("get_service_summary_today");
      } catch (err) {
        console.error(err);
      }
    },
  },
});
