import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";

/** Format Date object ke string YYYY-MM-DD (local time). */
function toDateString(date) {
  const y = date.getFullYear();
  const m = String(date.getMonth() + 1).padStart(2, "0");
  const d = String(date.getDate()).padStart(2, "0");
  return `${y}-${m}-${d}`;
}

/** Default range: 7 hari terakhir termasuk hari ini. */
function defaultRange() {
  const end = new Date();
  const start = new Date();
  start.setDate(start.getDate() - 6);
  return { start: toDateString(start), end: toDateString(end) };
}

export const useReportStore = defineStore("report", {
  state: () => ({
    startDate: defaultRange().start,
    endDate: defaultRange().end,

    revenue: { total_revenue: 0, total_transactions: 0, points: [] },
    topProducts: [],
    leastProducts: [],
    categoryReport: [],
    serviceTypeReport: { toko: 0, percetakan: 0, jasa_digital: 0 },
    stockAlerts: [],

    loading: false,
    error: null,
  }),

  actions: {
    setRange(start, end) {
      this.startDate = start;
      this.endDate = end;
    },

    /** Set range cepat: 7 hari, 30 hari, atau bulan ini. */
    setQuickRange(type) {
      const end = new Date();
      const start = new Date();

      if (type === "7d") {
        start.setDate(start.getDate() - 6);
      } else if (type === "30d") {
        start.setDate(start.getDate() - 29);
      } else if (type === "month") {
        start.setDate(1);
      } else if (type === "year") {
        start.setMonth(0, 1);
      }

      this.startDate = toDateString(start);
      this.endDate = toDateString(end);
    },

    /** Muat seluruh data laporan untuk rentang tanggal saat ini. */
    async fetchAll() {
      this.loading = true;
      this.error = null;
      try {
        const [
          revenue,
          topProducts,
          leastProducts,
          categoryReport,
          serviceTypeReport,
          stockAlerts,
        ] = await Promise.all([
          invoke("get_revenue_report", {
            startDate: this.startDate,
            endDate: this.endDate,
          }),
          invoke("get_top_products", {
            startDate: this.startDate,
            endDate: this.endDate,
            limit: 10,
            ascending: false,
          }),
          invoke("get_top_products", {
            startDate: this.startDate,
            endDate: this.endDate,
            limit: 10,
            ascending: true,
          }),
          invoke("get_category_report", {
            startDate: this.startDate,
            endDate: this.endDate,
          }),
          invoke("get_service_type_report", {
            startDate: this.startDate,
            endDate: this.endDate,
          }),
          invoke("get_stock_alert_report"),
        ]);

        this.revenue = revenue;
        this.topProducts = topProducts;
        this.leastProducts = leastProducts;
        this.categoryReport = categoryReport;
        this.serviceTypeReport = serviceTypeReport;
        this.stockAlerts = stockAlerts;
      } catch (err) {
        this.error = err?.toString() ?? "Gagal memuat laporan";
        console.error(err);
      } finally {
        this.loading = false;
      }
    },
  },
});
