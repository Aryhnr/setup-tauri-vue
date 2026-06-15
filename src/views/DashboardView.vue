<template>
  <div class="space-y-6 select-none">
    <!-- Welcome Banner -->
    <div class="bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-lg p-6 flex items-start gap-4">
      <div class="p-3 bg-blue-50 dark:bg-blue-900/20 rounded-md border border-blue-100 dark:border-blue-800/50">
        <SparklesIcon class="w-6 h-6 text-blue-600 dark:text-blue-400" />
      </div>
      <div>
        <h2 class="text-xl font-bold text-gray-900 dark:text-white mb-1">Selamat Datang, {{ storeName }}!</h2>
        <p class="text-sm font-medium text-gray-500 dark:text-gray-400">
          Mulai transaksi di menu
          <router-link to="/kasir" class="text-blue-600 dark:text-blue-400 hover:text-blue-700 dark:hover:text-blue-300 underline underline-offset-2 transition-colors">Kasir / POS</router-link>,
          atau kelola data produk di menu
          <router-link to="/produk" class="text-blue-600 dark:text-blue-400 hover:text-blue-700 dark:hover:text-blue-300 underline underline-offset-2 transition-colors">Produk & Stok</router-link>.
        </p>
      </div>
    </div>

    <!-- Stat Cards -->
    <div class="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-4 gap-4">
      <div class="bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-lg p-5">
        <div class="flex items-center justify-between mb-3">
          <p class="text-sm font-semibold text-gray-500 dark:text-gray-400">Omset Hari Ini</p>
          <BanknotesIcon class="w-5 h-5 text-green-500" />
        </div>
        <p class="text-2xl font-bold text-gray-900 dark:text-white">{{ formatRupiah(todayRevenue) }}</p>
        <p class="text-xs text-gray-400 mt-1">{{ todayTransactionCount }} transaksi</p>
      </div>

      <div class="bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-lg p-5">
        <div class="flex items-center justify-between mb-3">
          <p class="text-sm font-semibold text-gray-500 dark:text-gray-400">Omset Bulan Ini</p>
          <ChartBarIcon class="w-5 h-5 text-blue-500" />
        </div>
        <p class="text-2xl font-bold text-gray-900 dark:text-white">{{ formatRupiah(monthRevenue) }}</p>
        <p class="text-xs text-gray-400 mt-1">{{ monthTransactionCount }} transaksi</p>
      </div>

      <div class="bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-lg p-5">
        <div class="flex items-center justify-between mb-3">
          <p class="text-sm font-semibold text-gray-500 dark:text-gray-400">Stok Menipis</p>
          <ExclamationTriangleIcon class="w-5 h-5 text-amber-500" />
        </div>
        <p class="text-2xl font-bold text-amber-600 dark:text-amber-400">{{ productStore.lowStockProducts.length }}</p>
        <p class="text-xs text-gray-400 mt-1">dari {{ productStore.products.length }} produk</p>
      </div>

      <div class="bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-lg p-5">
        <div class="flex items-center justify-between mb-3">
          <p class="text-sm font-semibold text-gray-500 dark:text-gray-400">Total Produk</p>
          <ArchiveBoxIcon class="w-5 h-5 text-purple-500" />
        </div>
        <p class="text-2xl font-bold text-gray-900 dark:text-white">{{ productStore.products.length }}</p>
        <p class="text-xs text-gray-400 mt-1">terdaftar di sistem</p>
      </div>
    </div>

    <!-- Grafik Omset 7 Hari -->
    <div class="bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-lg">
      <div class="px-5 py-4 border-b border-gray-100 dark:border-gray-800 flex items-center justify-between">
        <div class="flex items-center gap-2">
          <ChartBarIcon class="w-5 h-5 text-gray-400" />
          <h3 class="font-bold text-gray-900 dark:text-white">Omset 7 Hari Terakhir</h3>
        </div>
        <div class="flex gap-1">
          <button
            class="text-xs px-3 py-1 rounded-md transition-colors"
            :class="chartType === 'bar' ? 'bg-blue-600 text-white' : 'bg-gray-100 dark:bg-gray-800 text-gray-500 dark:text-gray-400'"
            @click="chartType = 'bar'"
          >Batang</button>
          <button
            class="text-xs px-3 py-1 rounded-md transition-colors"
            :class="chartType === 'line' ? 'bg-blue-600 text-white' : 'bg-gray-100 dark:bg-gray-800 text-gray-500 dark:text-gray-400'"
            @click="chartType = 'line'"
          >Garis</button>
        </div>
      </div>
      <div class="p-5">
        <div class="relative h-52">
          <canvas ref="chartCanvas"></canvas>
        </div>
        <p v-if="revenuePoints.length === 0" class="text-center text-gray-400 text-sm mt-4">Belum ada data transaksi 7 hari terakhir</p>
      </div>
    </div>

    <!-- Baris bawah: Transaksi Terbaru + Stok Menipis -->
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
              <div>
                <p class="text-sm font-bold text-gray-900 dark:text-white">{{ p.name }}</p>
                <p class="text-xs text-gray-400">Min. stok: {{ p.min_stock }} {{ p.unit }}</p>
              </div>
              <div
                class="flex items-center border px-2.5 py-1 rounded-md"
                :class="p.stock === 0
                  ? 'border-red-200 dark:border-red-900/50 bg-red-50 dark:bg-red-900/20'
                  : 'border-amber-200 dark:border-amber-900/50 bg-amber-50 dark:bg-amber-900/20'"
              >
                <span
                  class="text-xs font-bold"
                  :class="p.stock === 0 ? 'text-red-600 dark:text-red-400' : 'text-amber-600 dark:text-amber-400'"
                >
                  {{ p.stock === 0 ? 'HABIS' : `Sisa: ${p.stock} ${p.unit}` }}
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
import { ref, computed, onMounted, watch, onBeforeUnmount } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useProductStore } from "../stores/productStore";
import { useCategoryStore } from "../stores/categoryStore";
import { useSettingStore } from "../stores/settingStore";
import {
  SparklesIcon,
  BanknotesIcon,
  ReceiptPercentIcon,
  ExclamationTriangleIcon,
  ArchiveBoxIcon,
  ClockIcon,
  ChartBarIcon,
} from "@heroicons/vue/24/outline";

const productStore = useProductStore();
const categoryStore = useCategoryStore();
const settingStore = useSettingStore();

const storeName = computed(() => settingStore.storeName || "Toko Madura");
const recentTransactions = ref([]);
const revenuePoints = ref([]); // [{ date, total, count }]
const chartCanvas = ref(null);
const chartType = ref("bar");
let chartInstance = null;
let ChartJS = null;

// ── Stat komputasi ──────────────────────────────────────────────────────────
const todayStr = new Date().toISOString().slice(0, 10);
const monthStr = new Date().toISOString().slice(0, 7); // YYYY-MM

const todayRevenue = computed(() =>
  recentTransactions.value
    .filter((tx) => tx.transaction_date?.slice(0, 10) === todayStr)
    .reduce((s, tx) => s + tx.total_amount, 0)
);

const todayTransactionCount = computed(() =>
  recentTransactions.value.filter((tx) => tx.transaction_date?.slice(0, 10) === todayStr).length
);

const monthRevenue = computed(() =>
  recentTransactions.value
    .filter((tx) => tx.transaction_date?.slice(0, 7) === monthStr)
    .reduce((s, tx) => s + tx.total_amount, 0)
);

const monthTransactionCount = computed(() =>
  recentTransactions.value.filter((tx) => tx.transaction_date?.slice(0, 7) === monthStr).length
);

// ── Format helpers ───────────────────────────────────────────────────────────
function formatRupiah(value) {
  return new Intl.NumberFormat("id-ID", { style: "currency", currency: "IDR", minimumFractionDigits: 0 }).format(value || 0);
}

function formatDate(dateStr) {
  if (!dateStr) return "-";
  const d = new Date(dateStr.replace(" ", "T") + "Z");
  return d.toLocaleString("id-ID", { day: "numeric", month: "short", hour: "2-digit", minute: "2-digit" });
}

// ── Grafik ───────────────────────────────────────────────────────────────────
async function renderChart() {
  if (!ChartJS) {
    const mod = await import("chart.js/auto");
    ChartJS = mod.default;
  }
  if (chartInstance) { chartInstance.destroy(); chartInstance = null; }
  if (!chartCanvas.value || revenuePoints.value.length === 0) return;

  const isDark = document.documentElement.classList.contains("dark");
  const gridColor = isDark ? "#374151" : "#f3f4f6";
  const tickColor = isDark ? "#9ca3af" : "#6b7280";

  const labels = revenuePoints.value.map((p) => {
    const d = new Date(`${p.date}T00:00:00`);
    return d.toLocaleDateString("id-ID", { day: "numeric", month: "short" });
  });
  const data = revenuePoints.value.map((p) => p.total);

  chartInstance = new ChartJS(chartCanvas.value, {
    type: chartType.value,
    data: {
      labels,
      datasets: [{
        label: "Omset",
        data,
        borderColor: "#2563eb",
        backgroundColor: chartType.value === "bar" ? "#3b82f680" : "#2563eb20",
        tension: 0.3,
        fill: chartType.value === "line",
        borderRadius: chartType.value === "bar" ? 4 : 0,
      }],
    },
    options: {
      responsive: true,
      maintainAspectRatio: false,
      plugins: {
        legend: { display: false },
        tooltip: {
          callbacks: {
            label: (ctx) => `Rp ${ctx.parsed.y.toLocaleString("id-ID")}`,
          },
        },
      },
      scales: {
        x: { grid: { color: gridColor }, ticks: { color: tickColor } },
        y: {
          grid: { color: gridColor },
          ticks: {
            color: tickColor,
            callback: (v) => `${(v / 1000).toLocaleString("id-ID")}rb`,
          },
        },
      },
    },
  });
}

watch([revenuePoints, chartType], renderChart, { deep: true });

onBeforeUnmount(() => chartInstance?.destroy());

// ── Data loading ─────────────────────────────────────────────────────────────
onMounted(async () => {
  await Promise.all([productStore.fetchAll(), categoryStore.fetchAll()]);
  try {
    recentTransactions.value = await invoke("get_recent_transactions", { limit: 30 });
  } catch (err) {
    console.error(err);
  }
  // Hitung revenue points 7 hari terakhir dari data transaksi
  buildRevenuePoints();
  await renderChart();
});

function buildRevenuePoints() {
  // Buat map tanggal → total 7 hari terakhir
  const points = [];
  for (let i = 6; i >= 0; i--) {
    const d = new Date();
    d.setDate(d.getDate() - i);
    const dateStr = d.toISOString().slice(0, 10);
    const total = recentTransactions.value
      .filter((tx) => tx.transaction_date?.slice(0, 10) === dateStr)
      .reduce((s, tx) => s + tx.total_amount, 0);
    const count = recentTransactions.value.filter((tx) => tx.transaction_date?.slice(0, 10) === dateStr).length;
    points.push({ date: dateStr, total, count });
  }
  revenuePoints.value = points;
}
</script>
