<template>
  <div class="space-y-4">
    <div class="card p-6">
      <h2 class="text-xl font-semibold mb-2">Selamat Datang, Toko Madura! 👋</h2>
      <p class="text-gray-500 text-sm">
        Dashboard lengkap (ringkasan omset, grafik, alert stok) akan dibangun pada
        <strong>Fase 2 - 4</strong>. Untuk saat ini, mulai kelola data produk di menu
        <router-link to="/produk" class="text-blue-600 hover:underline">Produk &amp; Stok</router-link>.
      </p>
    </div>

    <div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
      <div class="card p-4">
        <p class="text-sm text-gray-400">Total Produk</p>
        <p class="text-2xl font-bold mt-1">{{ productStore.products.length }}</p>
      </div>
      <div class="card p-4">
        <p class="text-sm text-gray-400">Stok Menipis</p>
        <p class="text-2xl font-bold mt-1 text-yellow-500">{{ productStore.lowStockProducts.length }}</p>
      </div>
      <div class="card p-4">
        <p class="text-sm text-gray-400">Kategori</p>
        <p class="text-2xl font-bold mt-1">{{ categoryStore.categories.length }}</p>
      </div>
    </div>
  </div>
</template>

<script setup>
import { onMounted } from "vue";
import { useProductStore } from "../stores/productStore";
import { useCategoryStore } from "../stores/categoryStore";

const productStore = useProductStore();
const categoryStore = useCategoryStore();

onMounted(async () => {
  await Promise.all([productStore.fetchAll(), categoryStore.fetchAll()]);
});
</script>