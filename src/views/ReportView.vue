<template>
  <div class="space-y-4">
    <!-- Filter tanggal -->
    <div class="card p-4 flex flex-col sm:flex-row sm:items-end gap-3">
      <div class="flex-1 grid grid-cols-2 gap-3">
        <div>
          <label class="label">Dari Tanggal</label>
          <input type="date" class="input" v-model="store.startDate" />
        </div>
        <div>
          <label class="label">Sampai Tanggal</label>
          <input type="date" class="input" v-model="store.endDate" />
        </div>
      </div>

      <div class="flex gap-2 flex-wrap">
        <button class="btn-secondary text-xs" @click="quickRange('7d')">7 Hari</button>
        <button class="btn-secondary text-xs" @click="quickRange('30d')">30 Hari</button>
        <button class="btn-secondary text-xs" @click="quickRange('month')">Bulan Ini</button>
        <button class="btn-secondary text-xs" @click="quickRange('year')">Tahun Ini</button>
        <BaseButton @click="store.fetchAll()">Terapkan</BaseButton>
      </div>
    </div>

    <!-- Tab navigasi -->
    <div class="flex gap-2 overflow-x-auto">
      <button
        v-for="tab in tabs"
        :key="tab.value"
        class="px-3 py-1.5 rounded-lg text-sm font-medium whitespace-nowrap transition-colors"
        :class="activeTab === tab.value
          ? 'bg-blue-600 text-white'
          : 'bg-gray-100 text-gray-600 dark:bg-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-600'"
        @click="activeTab = tab.value"
      >
        {{ tab.label }}
      </button>
    </div>

    <p v-if="store.loading" class="text-sm text-gray-400">Memuat laporan...</p>
    <p v-if="store.error" class="text-sm text-red-500">{{ store.error }}</p>

    <!-- Tab: Ringkasan -->
    <div v-if="activeTab === 'ringkasan'" class="space-y-4">
      <div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
        <div class="card p-4">
          <p class="text-sm text-gray-400">Total Omset</p>
          <p class="text-2xl font-bold mt-1">{{ formatRupiah(totalCombined) }}</p>
        </div>
        <div class="card p-4">
          <p class="text-sm text-gray-400">Jumlah Transaksi (POS)</p>
          <p class="text-2xl font-bold mt-1">{{ store.revenue.total_transactions }}</p>
        </div>
        <div class="card p-4">
          <p class="text-sm text-gray-400">Produk Stok Menipis/Habis</p>
          <p class="text-2xl font-bold mt-1 text-yellow-500">{{ store.stockAlerts.length }}</p>
        </div>
      </div>

      <RevenueChart :points="store.revenue.points" />

      <div class="card p-4">
        <h3 class="font-semibold mb-3">Pendapatan per Jenis Layanan</h3>
        <div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
          <div class="text-center">
            <p class="text-sm text-gray-400">Toko (Penjualan Barang)</p>
            <p class="text-xl font-bold mt-1">{{ formatRupiah(store.serviceTypeReport.toko) }}</p>
          </div>
          <div class="text-center">
            <p class="text-sm text-gray-400">Percetakan</p>
            <p class="text-xl font-bold mt-1">{{ formatRupiah(store.serviceTypeReport.percetakan) }}</p>
          </div>
          <div class="text-center">
            <p class="text-sm text-gray-400">Jasa Digital</p>
            <p class="text-xl font-bold mt-1">{{ formatRupiah(store.serviceTypeReport.jasa_digital) }}</p>
          </div>
        </div>
      </div>

      <div class="flex justify-end gap-2">
        <BaseButton variant="secondary" @click="exportRingkasanPdf">📄 Export PDF</BaseButton>
        <BaseButton variant="secondary" @click="exportRingkasanExcel">📊 Export Excel</BaseButton>
      </div>
    </div>

    <!-- Tab: Per Produk -->
    <div v-else-if="activeTab === 'produk'" class="space-y-4">
      <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
        <TopProductTable title="Produk Terlaris (Top 10)" :rows="store.topProducts" />
        <TopProductTable title="Produk Kurang Laku" :rows="store.leastProducts" />
      </div>

      <div v-if="store.stockAlerts.length > 0" class="card p-4">
        <h3 class="font-semibold mb-3">⚠️ Stok Menipis / Habis</h3>
        <table class="table-base">
          <thead>
            <tr>
              <th>Produk</th>
              <th class="text-right">Stok</th>
              <th class="text-right">Batas Minimum</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="row in store.stockAlerts" :key="row.id">
              <td class="font-medium">{{ row.name }}</td>
              <td class="text-right">
                <span
                  class="px-2 py-0.5 rounded-full text-xs font-semibold"
                  :class="row.stock === 0
                    ? 'bg-red-100 text-red-600 dark:bg-red-900/40 dark:text-red-300'
                    : 'bg-yellow-100 text-yellow-700 dark:bg-yellow-900/40 dark:text-yellow-300'"
                >
                  {{ row.stock }} {{ row.unit }}
                </span>
              </td>
              <td class="text-right">{{ row.min_stock }} {{ row.unit }}</td>
            </tr>
          </tbody>
        </table>
      </div>

      <div class="flex justify-end gap-2">
        <BaseButton variant="secondary" @click="exportProdukPdf">📄 Export PDF</BaseButton>
        <BaseButton variant="secondary" @click="exportProdukExcel">📊 Export Excel</BaseButton>
      </div>
    </div>

    <!-- Tab: Per Kategori -->
    <div v-else-if="activeTab === 'kategori'" class="space-y-4">
      <div class="card p-4">
        <h3 class="font-semibold mb-3">Penjualan per Kategori</h3>
        <div v-if="store.categoryReport.length === 0" class="text-center text-gray-400 text-sm py-6">
          Tidak ada data
        </div>
        <table v-else class="table-base">
          <thead>
            <tr>
              <th>Kategori</th>
              <th class="text-right">Terjual</th>
              <th class="text-right">Omset</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="row in store.categoryReport" :key="row.category_name">
              <td class="font-medium">{{ row.category_name }}</td>
              <td class="text-right">{{ row.total_quantity }}</td>
              <td class="text-right">{{ formatRupiah(row.total_revenue) }}</td>
            </tr>
          </tbody>
        </table>
      </div>

      <div class="flex justify-end gap-2">
        <BaseButton variant="secondary" @click="exportKategoriPdf">📄 Export PDF</BaseButton>
        <BaseButton variant="secondary" @click="exportKategoriExcel">📊 Export Excel</BaseButton>
      </div>
    </div>

    <!-- Tab: Percetakan -->
    <div v-else-if="activeTab === 'percetakan'" class="space-y-4">
      <div class="card p-4 text-center">
        <p class="text-sm text-gray-400">Total Pendapatan Percetakan (status Dibayar)</p>
        <p class="text-2xl font-bold mt-1">{{ formatRupiah(store.serviceTypeReport.percetakan) }}</p>
        <p class="text-xs text-gray-400 mt-2">
          Detail order per status tersedia di menu
          <router-link to="/percetakan" class="text-blue-600 hover:underline">Percetakan</router-link>.
        </p>
      </div>
    </div>

    <!-- Tab: Jasa -->
    <div v-else-if="activeTab === 'jasa'" class="space-y-4">
      <div class="card p-4 text-center">
        <p class="text-sm text-gray-400">Total Pendapatan Jasa Digital</p>
        <p class="text-2xl font-bold mt-1">{{ formatRupiah(store.serviceTypeReport.jasa_digital) }}</p>
        <p class="text-xs text-gray-400 mt-2">
          Detail per jenis layanan (Token PLN, Top Up, Pulsa, Paket Data) tersedia di menu
          <router-link to="/jasa" class="text-blue-600 hover:underline">Jasa & Layanan</router-link>.
        </p>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from "vue";
import { useReportStore } from "../stores/reportStore";
import { exportToPdf, exportToExcel } from "../composables/useExport";
import BaseButton from "../components/ui/BaseButton.vue";
import RevenueChart from "../components/report/RevenueChart.vue";
import TopProductTable from "../components/report/TopProductTable.vue";

const store = useReportStore();
const activeTab = ref("ringkasan");

const tabs = [
  { value: "ringkasan", label: "Ringkasan" },
  { value: "produk", label: "Per Produk" },
  { value: "kategori", label: "Per Kategori" },
  { value: "percetakan", label: "Percetakan" },
  { value: "jasa", label: "Jasa" },
];

const totalCombined = computed(() => {
  return (
    store.revenue.total_revenue +
    store.serviceTypeReport.percetakan +
    store.serviceTypeReport.jasa_digital
  );
});

function formatRupiah(value) {
  return new Intl.NumberFormat("id-ID", { style: "currency", currency: "IDR", minimumFractionDigits: 0 }).format(
    value || 0
  );
}

function quickRange(type) {
  store.setQuickRange(type);
  store.fetchAll();
}

const rangeLabel = computed(() => `Periode: ${store.startDate} s/d ${store.endDate}`);

// --- Export Ringkasan ---
function exportRingkasanExcel() {
  exportToExcel({
    filename: `laporan-ringkasan-${store.startDate}_${store.endDate}`,
    sheetName: "Ringkasan",
    rows: store.revenue.points,
    columns: [
      { key: "date", label: "Tanggal" },
      { key: "total", label: "Omset" },
      { key: "count", label: "Jumlah Transaksi" },
    ],
  });
}

function exportRingkasanPdf() {
  exportToPdf({
    filename: `laporan-ringkasan-${store.startDate}_${store.endDate}`,
    title: "Laporan Ringkasan Pendapatan",
    subtitle: rangeLabel.value,
    summary: [
      { label: "Total Omset (Toko)", value: formatRupiah(store.revenue.total_revenue) },
      { label: "Total Omset Percetakan", value: formatRupiah(store.serviceTypeReport.percetakan) },
      { label: "Total Omset Jasa Digital", value: formatRupiah(store.serviceTypeReport.jasa_digital) },
      { label: "Total Keseluruhan", value: formatRupiah(totalCombined.value) },
    ],
    columns: [
      { key: "date", label: "Tanggal" },
      { key: "total", label: "Omset" },
      { key: "count", label: "Jumlah Transaksi" },
    ],
    rows: store.revenue.points,
  });
}

// --- Export Per Produk ---
function exportProdukExcel() {
  exportToExcel({
    filename: `laporan-produk-${store.startDate}_${store.endDate}`,
    sheetName: "Produk Terlaris",
    rows: store.topProducts,
    columns: [
      { key: "name", label: "Produk" },
      { key: "total_quantity", label: "Terjual" },
      { key: "total_revenue", label: "Omset" },
    ],
  });
}

function exportProdukPdf() {
  exportToPdf({
    filename: `laporan-produk-${store.startDate}_${store.endDate}`,
    title: "Laporan Produk Terlaris",
    subtitle: rangeLabel.value,
    columns: [
      { key: "name", label: "Produk" },
      { key: "total_quantity", label: "Terjual" },
      { key: "total_revenue", label: "Omset" },
    ],
    rows: store.topProducts,
  });
}

// --- Export Per Kategori ---
function exportKategoriExcel() {
  exportToExcel({
    filename: `laporan-kategori-${store.startDate}_${store.endDate}`,
    sheetName: "Per Kategori",
    rows: store.categoryReport,
    columns: [
      { key: "category_name", label: "Kategori" },
      { key: "total_quantity", label: "Terjual" },
      { key: "total_revenue", label: "Omset" },
    ],
  });
}

function exportKategoriPdf() {
  exportToPdf({
    filename: `laporan-kategori-${store.startDate}_${store.endDate}`,
    title: "Laporan Penjualan per Kategori",
    subtitle: rangeLabel.value,
    columns: [
      { key: "category_name", label: "Kategori" },
      { key: "total_quantity", label: "Terjual" },
      { key: "total_revenue", label: "Omset" },
    ],
    rows: store.categoryReport,
  });
}

onMounted(() => {
  store.fetchAll();
});
</script>