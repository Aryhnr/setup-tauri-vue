import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";

export const useProductStore = defineStore("product", {
  state: () => ({
    products: [],
    loading: false,
    error: null,
  }),

  getters: {
    /**
     * Produk dengan stok di bawah atau sama dengan batas minimum.
     */
    lowStockProducts(state) {
      return state.products.filter((p) => p.stock <= p.min_stock);
    },
  },

  actions: {
    /**
     * Ambil seluruh produk, opsional filter berdasarkan keyword (nama / barcode).
     * @param {string} keyword
     */
    async fetchAll(keyword = "") {
      this.loading = true;
      this.error = null;
      try {
        this.products = await invoke("get_products", {
          keyword: keyword || null,
        });
      } catch (err) {
        this.error = err?.toString() ?? "Gagal memuat data produk";
        console.error(err);
      } finally {
        this.loading = false;
      }
    },

    /**
     * Tambah produk baru.
     * @param {Object} payload
     */
    async add(payload) {
      const product = {
        id: null,
        barcode: payload.barcode || null,
        name: payload.name,
        category_id: payload.category_id || null,
        buy_price: Number(payload.buy_price) || 0,
        sell_price: Number(payload.sell_price) || 0,
        stock: Number(payload.stock) || 0,
        min_stock: Number(payload.min_stock) || 0,
        unit: payload.unit || "pcs",
        supplier_id: payload.supplier_id || null,
        created_at: null,
        category_name: null,
        supplier_name: null,
      };
      await invoke("add_product", { product });
      await this.fetchAll();
    },

    /**
     * Update produk berdasarkan id.
     * @param {number} id
     * @param {Object} payload
     */
    async update(id, payload) {
      const product = {
        id,
        barcode: payload.barcode || null,
        name: payload.name,
        category_id: payload.category_id || null,
        buy_price: Number(payload.buy_price) || 0,
        sell_price: Number(payload.sell_price) || 0,
        stock: Number(payload.stock) || 0,
        min_stock: Number(payload.min_stock) || 0,
        unit: payload.unit || "pcs",
        supplier_id: payload.supplier_id || null,
        created_at: null,
        category_name: null,
        supplier_name: null,
      };
      await invoke("update_product", { id, product });
      await this.fetchAll();
    },

    /**
     * Hapus produk berdasarkan id.
     * @param {number} id
     */
    async remove(id) {
      await invoke("delete_product", { id });
      await this.fetchAll();
    },

    /**
     * Cari produk berdasarkan barcode (untuk Modul POS nanti).
     * @param {string} barcode
     */
    async findByBarcode(barcode) {
      return await invoke("find_product_by_barcode", { barcode });
    },
  },
});
