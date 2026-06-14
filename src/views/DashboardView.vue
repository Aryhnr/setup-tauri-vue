<template>
  <div class="space-y-6 select-none">
    <div class="bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-lg p-6 flex items-start gap-4">
      <div class="p-3 bg-blue-50 dark:bg-blue-900/20 rounded-md border border-blue-100 dark:border-blue-800/50">
        <SparklesIcon class="w-6 h-6 text-blue-600 dark:text-blue-400" />
      </div>
      <div>
        <h2 class="text-xl font-bold text-gray-900 dark:text-white mb-1">Selamat Datang, Toko Madura!</h2>
        <p class="text-sm font-medium text-gray-500 dark:text-gray-400">
          Mulai transaksi di menu
          <router-link to="/kasir" class="text-blue-600 dark:text-blue-400 hover:text-blue-700 dark:hover:text-blue-300 underline underline-offset-2 transition-colors">Kasir / POS</router-link>,
          atau kelola data produk di menu
          <router-link to="/produk" class="text-blue-600 dark:text-blue-400 hover:text-blue-700 dark:hover:text-blue-300 underline underline-offset-2 transition-colors">Produk & Stok</router-link>.
        </p>
      </div>
    </div>

    <div class="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-4 gap-4">
      <div class="bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-lg p-5">
        <div class="flex items-center justify-between mb-3">
          <p class="text-sm font-semibold text-gray-500 dark:text-gray-400">Omset Hari Ini</p>
          <BanknotesIcon class="w-5 h-5 text-green-500" />
        </div>
        <p class="text-2xl font-bold text-gray-900 dark:text-white">{{ formatRupiah(todayRevenue) }}</p>
      </div>

      <div class="bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-lg p-5">
        <div class="flex items-center justify-between mb-3">
          <p class="text-sm font-semibold text-gray-500 dark:text-gray-400">Transaksi Hari Ini</p>
          <ReceiptPercentIcon class="w-5 h-5 text-blue-500" />
        </div>
        <p class="text-2xl font-bold text-gray-900 dark:text-white">{{ todayTransactionCount }}</p>
      </div>

      <div class="bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-lg p-5">
        <div class="flex items-center justify-between mb-3">
          <p class="text-sm font-semibold text-gray-500 dark:text-gray-400">Stok Menipis</p>
          <ExclamationTriangleIcon class="w-5 h-5 text-amber-500" />
        </div>
        <p class="text-2xl font-bold text-amber-600 dark:text-amber-400">{{ productStore.lowStockProducts.length }}</p>
      </div>

      <div class="bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-lg p-5">
        <div class="flex items-center justify-between mb-3">
          <p class="text-sm font-semibold text-gray-500 dark:text-gray-400">Total Produk</p>
          <ArchiveBoxIcon class="w-5 h-5 text-purple-500" />
        </div>
        <p class="text-2xl font-bold text-gray-900 dark:text-white">{{ productStore.products.length }}</p>
      </div>
    </div>

    <div class="grid grid-cols-1 xl:grid-cols-2 gap-4">
      
      <div class="bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-lg flex flex-col">
        <div class="px-5 py-4 border-b border-gray-100 dark:border-gray-800 flex items-center gap-2">
          <ClockIcon class="w-5 h-5 text-gray-400" />
          <h3 class="font-bold text-gray-900 dark:text-white">Transaksi Terbaru</h3>
        </div>
        
        <div class="p-5 flex-1">
          <div v-if="recentTransactions.length === 0" class="h-full flex flex-col items-center justify-center text-gray-400 py-6">
            <ReceiptPercentIcon class="w-8 h-8 mb-2 opacity-50" />
            <p class="text-sm font-medium">Belum ada transaksi</p>
          </div>
          
          <div v-else class="space-y-1">
            <div
              v-for="tx in recentTransactions"
              :key="tx.id"
              class="flex items-center justify-between px-2 py-2.5 hover:bg-gray-50 dark:hover:bg-gray-800/50 rounded-md transition-colors"
            >
              <div>
                <p class="text-sm font-bold text-gray-900 dark:text-white">{{ tx.invoice_no }}</p>
                <p class="text-xs font-medium text-gray-500">{{ formatDate(tx.transaction_date) }}</p>
              </div>
              <p class="text-sm font-bold text-gray-900 dark:text-white">{{ formatRupiah(tx.total_amount) }}</p>
            </div>
          </div>
        </div>
      </div>

      <div class="bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-lg flex flex-col">
        <div class="px-5 py-4 border-b border-gray-100 dark:border-gray-800 flex items-center gap-2">
          <ExclamationTriangleIcon class="w-5 h-5 text-amber-500" />
          <h3 class="font-bold text-gray-900 dark:text-white">Peringatan Stok</h3>
        </div>
        
        <div class="p-5 flex-1">
          <div v-if="productStore.lowStockProducts.length === 0" class="h-full flex flex-col items-center justify-center text-gray-400 py-6">
            <ArchiveBoxIcon class="w-8 h-8 mb-2 opacity-50 text-green-500" />
            <p class="text-sm font-medium">Semua stok produk aman</p>
          </div>
          
          <div v-else class="space-y-1">
            <div
              v-for="p in productStore.lowStockProducts"
              :key="p.id"
              class="flex items-center justify-between px-2 py-2.5 hover:bg-gray-50 dark:hover:bg-gray-800/50 rounded-md transition-colors"
            >
              <p class="text-sm font-bold text-gray-900 dark:text-white">{{ p.name }}</p>
              <div class="flex items-center border border-amber-200 dark:border-amber-900/50 bg-amber-50 dark:bg-amber-900/20 px-2.5 py-1 rounded-md">
                <span class="text-xs font-bold text-amber-600 dark:text-amber-400">
                  Sisa: {{ p.stock }} {{ p.unit }}
                </span>
              </div>
            </div>
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
import { 
  SparklesIcon, 
  BanknotesIcon, 
  ReceiptPercentIcon, 
  ExclamationTriangleIcon, 
  ArchiveBoxIcon, 
  ClockIcon 
} from "@heroicons/vue/24/outline";

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