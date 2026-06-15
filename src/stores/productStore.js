import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";
import { useToast } from "../composables/useToast";

export const useProductStore = defineStore("product", {
  state: () => ({
    products: [],
    loading: false,
    error: null,
  }),

  getters: {
    lowStockProducts(state) {
      return state.products.filter((p) => p.stock <= p.min_stock);
    },
  },

  actions: {
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

    async add(payload) {
      const toast = useToast();
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
      toast.success(`Produk "${payload.name}" berhasil ditambahkan`);
    },

    async update(id, payload) {
      const toast = useToast();
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
      toast.success(`Produk "${payload.name}" berhasil diperbarui`);
    },

    async remove(id) {
      const toast = useToast();
      const name = this.products.find((p) => p.id === id)?.name ?? "Produk";
      await invoke("delete_product", { id });
      await this.fetchAll();
      toast.success(`"${name}" berhasil dihapus`);
    },

    async findByBarcode(barcode) {
      return await invoke("find_product_by_barcode", { barcode });
    },
  },
});
