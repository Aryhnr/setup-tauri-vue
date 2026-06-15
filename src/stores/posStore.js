import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";
import { useNotification } from "../composables/useNotification";
import { useToast } from "../composables/useToast";

export const usePosStore = defineStore("pos", {
  state: () => ({
    cart: [],
    paidAmount: 0,
    processing: false,
    error: null,
    lastResult: null, // { id, invoice_no }
  }),

  getters: {
    total(state) {
      return state.cart.reduce((sum, item) => sum + item.subtotal, 0);
    },

    changeAmount(state) {
      const total = state.cart.reduce((sum, item) => sum + item.subtotal, 0);
      const change = Number(state.paidAmount) - total;
      return change > 0 ? change : 0;
    },

    isPaidEnough(state) {
      const total = state.cart.reduce((sum, item) => sum + item.subtotal, 0);
      return Number(state.paidAmount) >= total && total > 0;
    },
  },

  actions: {
    addProduct(product, qty = 1) {
      const existing = this.cart.find((item) => item.product_id === product.id);

      if (existing) {
        existing.quantity += qty;
        existing.subtotal = existing.quantity * existing.unit_price;
      } else {
        this.cart.push({
          product_id: product.id,
          item_name: product.name,
          quantity: qty,
          unit_price: product.sell_price,
          subtotal: product.sell_price * qty,
          stock: product.stock,
          unit: product.unit,
        });
      }
    },

    addCustomItem(item) {
      const quantity = Number(item.quantity) || 1;
      const unit_price = Number(item.unit_price) || 0;
      this.cart.push({
        product_id: null,
        item_name: item.item_name,
        quantity,
        unit_price,
        subtotal: quantity * unit_price,
        stock: null,
        unit: "",
      });
    },

    updateQuantity(index, qty) {
      const item = this.cart[index];
      if (!item) return;

      let newQty = Math.max(1, Math.floor(Number(qty) || 1));
      if (item.stock !== null && item.stock !== undefined) {
        newQty = Math.min(newQty, item.stock);
      }

      item.quantity = newQty;
      item.subtotal = item.quantity * item.unit_price;
    },

    removeItem(index) {
      const toast = useToast();
      const name = this.cart[index]?.item_name ?? "Item";
      this.cart.splice(index, 1);
      toast.info(`${name} dihapus dari keranjang`);
    },

    clearCart() {
      this.cart = [];
      this.paidAmount = 0;
      this.error = null;
      this.lastResult = null;
    },

    async checkout() {
      const toast = useToast();
      this.error = null;

      if (this.cart.length === 0) {
        this.error = "Keranjang masih kosong";
        toast.warning("Keranjang masih kosong");
        return false;
      }

      if (!this.isPaidEnough) {
        this.error = "Jumlah bayar belum cukup";
        toast.error("Jumlah bayar belum cukup");
        return false;
      }

      this.processing = true;
      try {
        const payload = {
          transaction_type: "PENJUALAN",
          total_amount: this.total,
          paid_amount: Number(this.paidAmount),
          change_amount: this.changeAmount,
          notes: null,
          items: this.cart.map((item) => ({
            product_id: item.product_id,
            item_name: item.item_name,
            quantity: item.quantity,
            unit_price: item.unit_price,
            subtotal: item.subtotal,
          })),
        };

        this.lastResult = await invoke("process_transaction", { payload });
        this.cart = [];
        this.paidAmount = 0;
        toast.success(
          `Transaksi ${this.lastResult?.invoice_no ?? ""} berhasil!`,
        );
        const { checkLowStock } = useNotification();
        checkLowStock();
        return true;
      } catch (err) {
        this.error = err?.toString() ?? "Gagal memproses transaksi";
        toast.error("Gagal memproses transaksi: " + (err?.toString() ?? ""));
        return false;
      } finally {
        this.processing = false;
      }
    },

    async checkoutAsDebt(debtInfo) {
      const { success, error } = useToast();
      this.error = null;

      if (this.cart.length === 0) {
        this.error = "Keranjang masih kosong";
        return false;
      }

      this.processing = true;
      try {
        const paidUpfront = Number(debtInfo.paid_upfront) || 0;
        const debtAmount = Number(debtInfo.debt_amount) || this.total;

        const payload = {
          transaction_type: "PENJUALAN",
          total_amount: this.total,
          paid_amount: paidUpfront,
          change_amount: 0,
          notes: `HUTANG - ${debtInfo.name}${paidUpfront > 0 ? ` (DP: Rp${paidUpfront.toLocaleString("id-ID")})` : ""}`,
          items: this.cart.map((item) => ({
            product_id: item.product_id,
            item_name: item.item_name,
            quantity: item.quantity,
            unit_price: item.unit_price,
            subtotal: item.subtotal,
          })),
        };

        this.lastResult = await invoke("process_transaction", { payload });

        // Catat hutang sebesar sisa yang belum dibayar
        await invoke("add_debt", {
          payload: {
            name: debtInfo.name,
            phone: debtInfo.phone || null,
            amount: debtAmount,
            notes: debtInfo.notes
              ? `${debtInfo.notes} | Invoice: ${this.lastResult.invoice_no}${paidUpfront > 0 ? ` | DP: Rp${paidUpfront.toLocaleString("id-ID")}` : ""}`
              : `Invoice: ${this.lastResult.invoice_no}${paidUpfront > 0 ? ` | DP: Rp${paidUpfront.toLocaleString("id-ID")}` : ""}`,
            due_date: debtInfo.due_date || null,
          },
        });

        this.cart = [];
        this.paidAmount = 0;

        const msg =
          paidUpfront > 0
            ? `Transaksi berhasil! DP Rp${paidUpfront.toLocaleString("id-ID")}, hutang ${debtInfo.name} Rp${debtAmount.toLocaleString("id-ID")}`
            : `Transaksi & hutang ${debtInfo.name} berhasil dicatat!`;

        success(msg);
        const { checkLowStock } = useNotification();
        checkLowStock();
        return true;
      } catch (err) {
        this.error = err?.toString() ?? "Gagal memproses";
        error("Gagal: " + this.error);
        return false;
      } finally {
        this.processing = false;
      }
    },
  },
});
