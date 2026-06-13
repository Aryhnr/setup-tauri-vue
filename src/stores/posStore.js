import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";

export const usePosStore = defineStore("pos", {
  state: () => ({
    /**
     * Item keranjang: { product_id, item_name, quantity, unit_price, subtotal, stock }
     * `stock` disimpan untuk validasi UI (tidak dikirim ke backend).
     */
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
    /**
     * Tambah produk ke keranjang. Jika produk sudah ada, tambah quantity.
     * @param {Object} product - hasil dari productStore (punya id, name, sell_price, stock, unit)
     * @param {number} qty
     */
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

    /**
     * Tambah item non-produk (percetakan/jasa) ke keranjang.
     * @param {Object} item - { item_name, quantity, unit_price }
     */
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

    /**
     * Update quantity item di keranjang berdasarkan index.
     * Quantity minimal 1, dan tidak boleh melebihi stok (untuk item produk).
     */
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
      this.cart.splice(index, 1);
    },

    clearCart() {
      this.cart = [];
      this.paidAmount = 0;
      this.error = null;
      this.lastResult = null;
    },

    /**
     * Kirim transaksi ke backend. Mengurangi stok produk secara atomic.
     * @returns {Promise<boolean>} true jika sukses
     */
    async checkout() {
      this.error = null;

      if (this.cart.length === 0) {
        this.error = "Keranjang masih kosong";
        return false;
      }

      if (!this.isPaidEnough) {
        this.error = "Jumlah bayar belum cukup";
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
        return true;
      } catch (err) {
        this.error = err?.toString() ?? "Gagal memproses transaksi";
        return false;
      } finally {
        this.processing = false;
      }
    },
  },
});
