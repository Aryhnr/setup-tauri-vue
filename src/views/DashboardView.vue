<template>
  <div class="space-y-4">
    <div class="card p-6">
      <h2 class="text-xl font-semibold mb-2">Selamat Datang, Toko Madura! 👋</h2>
      <p class="text-gray-500 text-sm">
        Mulai transaksi di menu
        <router-link to="/kasir" class="text-blue-600 hover:underline">Kasir / POS</router-link>,
        atau kelola data produk di menu
        <router-link to="/produk" class="text-blue-600 hover:underline">Produk &amp; Stok</router-link>.
      </p>
    </div>

    <div class="grid grid-cols-1 sm:grid-cols-4 gap-4">
      <div class="card p-4">
        <p class="text-sm text-gray-400">Omset Hari Ini</p>
        <p class="text-2xl font-bold mt-1">{{ formatRupiah(todayRevenue) }}</p>
      </div>
      <div class="card p-4">
        <p class="text-sm text-gray-400">Transaksi Hari Ini</p>
        <p class="text-2xl font-bold mt-1">{{ todayTransactionCount }}</p>
      </div>
      <div class="card p-4">
        <p class="text-sm text-gray-400">Stok Menipis</p>
        <p class="text-2xl font-bold mt-1 text-yellow-500">{{ productStore.lowStockProducts.length }}</p>
      </div>
      <div class="card p-4">
        <p class="text-sm text-gray-400">Total Produk</p>
        <p class="text-2xl font-bold mt-1">{{ productStore.products.length }}</p>
      </div>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
      <div class="card p-4">
        <h3 class="font-semibold mb-3">Transaksi Terbaru</h3>
        <div v-if="recentTransactions.length === 0" class="text-center text-gray-400 text-sm py-6">
          Belum ada transaksi
        </div>
        <div v-else class="space-y-2">
          <div
            v-for="tx in recentTransactions"
            :key="tx.id"
            class="flex items-center justify-between text-sm py-1.5 border-b border-gray-100 dark:border-gray-700 last:border-b-0"
          >
            <div>
              <p class="font-medium">{{ tx.invoice_no }}</p>
              <p class="text-xs text-gray-400">{{ formatDate(tx.transaction_date) }}</p>
            </div>
            <p class="font-semibold">{{ formatRupiah(tx.total_amount) }}</p>
          </div>
        </div>
      </div>

      <div class="card p-4">
        <h3 class="font-semibold mb-3">⚠️ Stok Menipis</h3>
        <div v-if="productStore.lowStockProducts.length === 0" class="text-center text-gray-400 text-sm py-6">
          Semua stok aman
        </div>
        <div v-else class="space-y-2">
          <div
            v-for="p in productStore.lowStockProducts"
            :key="p.id"
            class="flex items-center justify-between text-sm py-1.5 border-b border-gray-100 dark:border-gray-700 last:border-b-0"
          >
            <p class="font-medium">{{ p.name }}</p>
            <span class="text-xs px-2 py-0.5 rounded-full bg-red-100 text-red-600 dark:bg-red-900/40 dark:text-red-300">
              Sisa: {{ p.stock }} {{ p.unit }}
            </span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useProductStore } from "../stores/productStore";
import { useCategoryStore } from "../stores/categoryStore";

const productStore = useProductStore();
const categoryStore = useCategoryStore();

const recentTransactions = ref([]);

const todayRevenue = computed(() => {
  const today = new Date().toISOString().slice(0, 10);
  return recentTransactions.value
    .filter((tx) => tx.transaction_date?.slice(0, 10) === today)
    .reduce((sum, tx) => sum + tx.total_amount, 0);
});

const todayTransactionCount = computed(() => {
  const today = new Date().toISOString().slice(0, 10);
  return recentTransactions.value.filter((tx) => tx.transaction_date?.slice(0, 10) === today).length;
});

function formatRupiah(value) {
  return new Intl.NumberFormat("id-ID", { style: "currency", currency: "IDR", minimumFractionDigits: 0 }).format(
    value || 0
  );
}

function formatDate(dateStr) {
  if (!dateStr) return "-";
  const d = new Date(dateStr.replace(" ", "T") + "Z");
  return d.toLocaleString("id-ID", { day: "numeric", month: "short", hour: "2-digit", minute: "2-digit" });
}

onMounted(async () => {
  await Promise.all([productStore.fetchAll(), categoryStore.fetchAll()]);
  try {
    recentTransactions.value = await invoke("get_recent_transactions", { limit: 10 });
  } catch (err) {
    console.error(err);
  }
});
</script>
