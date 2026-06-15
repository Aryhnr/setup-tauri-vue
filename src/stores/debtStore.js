import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";
import { useToast } from "../composables/useToast";

export const useDebtStore = defineStore("debt", {
  state: () => ({
    debts: [],
    summary: { count: 0, total_amount: 0, total_remaining: 0 },
    loading: false,
    error: null,
  }),

  actions: {
    async fetchAll(status = null) {
      this.loading = true;
      this.error = null;
      try {
        this.debts = await invoke("get_debts", { status });
      } catch (err) {
        this.error = err?.toString() ?? "Gagal memuat data hutang";
      } finally {
        this.loading = false;
      }
    },

    async fetchSummary() {
      try {
        this.summary = await invoke("get_debt_summary");
      } catch (err) {
        console.error(err);
      }
    },

    async add(payload) {
      const { success, error } = useToast();
      try {
        await invoke("add_debt", { payload });
        await this.fetchAll();
        await this.fetchSummary();
        success(`Hutang "${payload.name}" berhasil dicatat`);
      } catch (err) {
        error("Gagal mencatat hutang: " + err);
        throw err;
      }
    },

    async addPayment(debtId, amount, notes = null) {
      const { success, error } = useToast();
      try {
        await invoke("add_debt_payment", { debtId, amount, notes });
        await this.fetchAll();
        await this.fetchSummary();
        success("Pembayaran berhasil dicatat");
      } catch (err) {
        error("Gagal mencatat pembayaran: " + err);
        throw err;
      }
    },

    async remove(id) {
      const { success, error } = useToast();
      try {
        await invoke("delete_debt", { id });
        await this.fetchAll();
        await this.fetchSummary();
        success("Data hutang berhasil dihapus");
      } catch (err) {
        error("Gagal menghapus hutang: " + err);
        throw err;
      }
    },

    async getPayments(debtId) {
      return await invoke("get_debt_payments", { debtId });
    },
  },
});
