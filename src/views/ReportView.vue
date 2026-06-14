<template>
  <div class="space-y-4">
    <!-- Filter Kontrol Periode & Tanggal -->
    <div class="bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-lg p-4 flex flex-col lg:flex-row lg:items-end gap-4 select-none">
      <div class="flex-1 grid grid-cols-1 sm:grid-cols-2 gap-4">
        <div>
          <label class="block text-xs font-bold text-gray-400 dark:text-gray-500 uppercase tracking-wider mb-1.5">Dari Tanggal</label>
          <div class="relative">
            <input 
              type="date" 
              class="w-full bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-md pl-3 pr-10 py-1.5 text-sm font-medium text-gray-700 dark:text-gray-300 focus:outline-none focus:border-indigo-500 dark:focus:border-indigo-400 transition-colors" 
              v-model="store.startDate" 
            />
          </div>
        </div>
        <div>
          <label class="block text-xs font-bold text-gray-400 dark:text-gray-500 uppercase tracking-wider mb-1.5">Sampai Tanggal</label>
          <div class="relative">
            <input 
              type="date" 
              class="w-full bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-md pl-3 pr-10 py-1.5 text-sm font-medium text-gray-700 dark:text-gray-300 focus:outline-none focus:border-indigo-500 dark:focus:border-indigo-400 transition-colors" 
              v-model="store.endDate" 
            />
          </div>
        </div>
      </div>

      <!-- Quick Ranges & Apply Action -->
      <div class="flex flex-wrap items-center sm:justify-end lg:justify-start gap-2">
        <div class="flex border border-gray-200 dark:border-gray-800 rounded-md bg-gray-50 dark:bg-gray-950 p-0.5">
          <button 
            v-for="range in [['7d', '7 Hari'], ['30d', '30 H'], ['month', 'Bulan Ini'], ['year', 'Tahun']]"
            :key="range[0]"
            class="px-2.5 py-1 text-[11px] font-bold text-gray-500 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white rounded-sm hover:bg-white dark:hover:bg-gray-900 transition-all border border-transparent hover:border-gray-200 dark:hover:border-gray-800"
            @click="quickRange(range[0])"
          >
            {{ range[1] }}
          </button>
        </div>
        <BaseButton class="flex items-center gap-1.5 py-2" @click="store.fetchAll()">
          <CalendarDaysIcon class="w-4 h-4" />
          <span>Terapkan</span>
        </BaseButton>
      </div>
    </div>

    <!-- Segmented Control Navigation Tabs -->
    <div class="flex border border-gray-200 dark:border-gray-800 p-1 rounded-lg bg-gray-50 dark:bg-gray-950 overflow-x-auto invisible-scrollbar select-none">
      <button
        v-for="tab in tabs"
        :key="tab.value"
        class="px-4 py-1.5 rounded-md text-xs font-bold whitespace-nowrap border transition-all"
        :class="activeTab === tab.value
          ? 'bg-white border-gray-200 text-indigo-600 dark:bg-gray-900 dark:border-gray-800 dark:text-indigo-400 font-extrabold'
          : 'bg-transparent border-transparent text-gray-500 dark:text-gray-400 hover:text-gray-800 dark:hover:text-gray-200'"
        @click="activeTab = tab.value"
      >
        {{ tab.label }}
      </button>
    </div>

    <!-- State Status Info -->
    <div class="flex flex-col gap-1 text-xs font-medium select-none px-1">
      <p v-if="store.loading" class="text-gray-400">Menghimpun matriks laporan finansial...</p>
      <p v-if="store.error" class="text-red-500 font-bold">{{ store.error }}</p>
    </div>

    <!-- Tab Content: Ringkasan -->
    <div v-if="activeTab === 'ringkasan'" class="space-y-4">
      <div class="grid grid-cols-1 sm:grid-cols-3 gap-4 select-none">
        <div class="bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-lg p-4 flex items-center justify-between">
          <div>
            <p class="text-[10px] font-black text-gray-400 dark:text-gray-500 uppercase tracking-wider">Total Akumulasi Omset</p>
            <p class="text-xl font-mono font-black text-gray-800 dark:text-white mt-1">{{ formatRupiah(totalCombined) }}</p>
          </div>
          <div class="p-2 bg-gray-50 dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-md text-gray-400">
            <PresentationChartLineIcon class="w-5 h-5" />
          </div>
        </div>

        <div class="bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-lg p-4 flex items-center justify-between">
          <div>
            <p class="text-[10px] font-black text-gray-400 dark:text-gray-500 uppercase tracking-wider">Transaksi Terproses (POS)</p>
            <p class="text-xl font-mono font-black text-gray-800 dark:text-white mt-1">{{ store.revenue.total_transactions }}</p>
          </div>
          <div class="p-2 bg-gray-50 dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-md text-gray-400">
            <DocumentCheckIcon class="w-5 h-5" />
          </div>
        </div>

        <div class="bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-lg p-4 flex items-center justify-between">
          <div>
            <p class="text-[10px] font-black text-gray-400 dark:text-gray-500 uppercase tracking-wider">Stok Menipis / Habis</p>
            <p class="text-xl font-mono font-black mt-1" :class="store.stockAlerts.length > 0 ? 'text-amber-600 dark:text-amber-400 font-extrabold' : 'text-gray-400'">
              {{ store.stockAlerts.length }}
            </p>
          </div>
          <div class="p-2 border rounded-md transition-colors" :class="store.stockAlerts.length > 0 ? 'bg-amber-50/50 dark:bg-amber-950/20 border-amber-100/70 dark:border-amber-900/40 text-amber-600 dark:text-amber-400' : 'bg-gray-50 dark:bg-gray-800 border-gray-200 dark:border-gray-700 text-gray-400'">
            <ExclamationTriangleIcon class="w-5 h-5" />
          </div>
        </div>
      </div>

      <!-- Chart Component -->
      <RevenueChart :points="store.revenue.points" />

      <!-- Pendapatan Berdasarkan Klasifikasi Layanan -->
      <div class="bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-lg p-4 select-none">
        <h3 class="text-xs font-bold text-gray-800 dark:text-gray-200 uppercase tracking-wider mb-4">Proporsi Omset Sektor Jasa & Toko</h3>
        <div class="grid grid-cols-1 sm:grid-cols-3 border border-gray-100 dark:border-gray-800 rounded-lg bg-gray-50 dark:bg-gray-950/50 divide-y sm:divide-y-0 sm:divide-x divide-gray-200 dark:divide-gray-800">
          <div class="p-4 text-center">
            <p class="text-[10px] font-bold text-gray-400 dark:text-gray-500 uppercase tracking-wider flex items-center justify-center gap-1">
              <ShoppingBagIcon class="w-3.5 h-3.5" /> Retail Barang (Toko)
            </p>
            <p class="text-lg font-mono font-black text-gray-800 dark:text-white mt-1">{{ formatRupiah(store.serviceTypeReport.toko) }}</p>
          </div>
          <div class="p-4 text-center">
            <p class="text-[10px] font-bold text-gray-400 dark:text-gray-500 uppercase tracking-wider flex items-center justify-center gap-1">
              <PrinterIcon class="w-3.5 h-3.5" /> Produksi Percetakan
            </p>
            <p class="text-lg font-mono font-black text-gray-800 dark:text-white mt-1">{{ formatRupiah(store.serviceTypeReport.percetakan) }}</p>
          </div>
          <div class="p-4 text-center">
            <p class="text-[10px] font-bold text-gray-400 dark:text-gray-500 uppercase tracking-wider flex items-center justify-center gap-1">
              <CpuChipIcon class="w-3.5 h-3.5" /> Transaksi Digital PPOB
            </p>
            <p class="text-lg font-mono font-black text-gray-800 dark:text-white mt-1">{{ formatRupiah(store.serviceTypeReport.jasa_digital) }}</p>
          </div>
        </div>
      </div>

      <!-- Aksi Ekspor Data Ringkasan -->
      <div class="flex justify-end gap-2 select-none">
        <BaseButton variant="secondary" class="flex items-center gap-1.5 text-xs font-bold" @click="exportRingkasanPdf">
          <DocumentArrowDownIcon class="w-3.5 h-3.5 text-rose-500/80" />
          <span>Ekspor PDF</span>
        </BaseButton>
        <BaseButton variant="secondary" class="flex items-center gap-1.5 text-xs font-bold" @click="exportRingkasanExcel">
          <ArrowDownTrayIcon class="w-3.5 h-3.5 text-emerald-600/80" />
          <span>Ekspor Excel</span>
        </BaseButton>
      </div>
    </div>

    <!-- Tab Content: Per Produk -->
    <div v-else-if="activeTab === 'produk'" class="space-y-4">
      <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
        <TopProductTable title="Produk Terlaris (Top 10)" :rows="store.topProducts" />
        <TopProductTable title="Produk Kurang Laku" :rows="store.leastProducts" />
      </div>

      <!-- Tabel Peringatan Ketersediaan Stok (Soft Style) -->
      <div v-if="store.stockAlerts.length > 0" class="bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-lg p-4">
        <h3 class="text-xs font-bold text-amber-600 dark:text-amber-400 uppercase tracking-wider mb-3 flex items-center gap-1.5 select-none">
          <ExclamationTriangleIcon class="w-4 h-4" /> Log Indikator Stok Menipis & Habis
        </h3>
        <div class="overflow-x-auto">
          <table class="w-full text-left text-sm border-collapse">
            <thead>
              <tr class="border-b border-gray-200 dark:border-gray-800 text-[11px] font-bold text-gray-400 uppercase tracking-wider select-none">
                <th class="pb-2 font-black">Spesifikasi Komoditas / Produk</th>
                <th class="pb-2 text-right font-black">Stok Aktual</th>
                <th class="pb-2 text-right font-black">Batas Threshold</th>
              </tr>
            </thead>
            <tbody class="divide-y divide-gray-100 dark:divide-gray-800/50">
              <tr v-for="row in store.stockAlerts" :key="row.id" class="text-xs text-gray-600 dark:text-gray-400">
                <td class="py-2.5 font-bold text-gray-800 dark:text-white">{{ row.name }}</td>
                <td class="py-2.5 text-right select-none">
                  <span
                    class="px-2 py-0.5 border rounded font-mono font-bold text-[10px]"
                    :class="row.stock === 0
                      ? 'bg-rose-50 text-rose-600 border-rose-100 dark:bg-rose-950/20 dark:text-rose-400 dark:border-rose-900/30'
                      : 'bg-amber-50 text-amber-700 border-amber-100 dark:bg-amber-950/20 dark:text-amber-400 dark:border-amber-900/30'"
                  >
                    {{ row.stock }} {{ row.unit }}
                  </span>
                </td>
                <td class="py-2.5 text-right font-mono text-gray-400 dark:text-gray-500 select-none">{{ row.min_stock }} {{ row.unit }}</td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>

      <!-- Aksi Ekspor Data Produk -->
      <div class="flex justify-end gap-2 select-none">
        <BaseButton variant="secondary" class="flex items-center gap-1.5 text-xs font-bold" @click="exportProdukPdf">
          <DocumentArrowDownIcon class="w-3.5 h-3.5 text-rose-500/80" />
          <span>Ekspor PDF</span>
        </BaseButton>
        <BaseButton variant="secondary" class="flex items-center gap-1.5 text-xs font-bold" @click="exportProdukExcel">
          <ArrowDownTrayIcon class="w-3.5 h-3.5 text-emerald-600/80" />
          <span>Ekspor Excel</span>
        </BaseButton>
      </div>
    </div>

    <!-- Tab Content: Per Kategori -->
    <div v-else-if="activeTab === 'kategori'" class="space-y-4">
      <div class="bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-lg p-4">
        <h3 class="text-xs font-bold text-gray-800 dark:text-gray-200 uppercase tracking-wider mb-3 flex items-center gap-1.5 select-none">
          <TagIcon class="w-4 h-4 text-gray-400" /> Kuantitas Distribusi Penjualan per Kategori
        </h3>
        
        <div v-if="store.categoryReport.length === 0" class="text-center text-gray-400 text-xs py-10 select-none">
          Tidak ada persebaran data dalam rentang tanggal terpilih
        </div>
        
        <div v-else class="overflow-x-auto">
          <table class="w-full text-left text-sm border-collapse">
            <thead>
              <tr class="border-b border-gray-200 dark:border-gray-800 text-[11px] font-bold text-gray-400 uppercase tracking-wider select-none">
                <th class="pb-2 font-black">Nama Kategori</th>
                <th class="pb-2 text-right font-black">Volume Terjual</th>
                <th class="pb-2 text-right font-black">Akumulasi Omset</th>
              </tr>
            </thead>
            <tbody class="divide-y divide-gray-100 dark:divide-gray-800/50">
              <tr v-for="row in store.categoryReport" :key="row.category_name" class="text-xs text-gray-600 dark:text-gray-400">
                <td class="py-2.5 font-bold text-gray-800 dark:text-white">{{ row.category_name }}</td>
                <td class="py-2.5 text-right font-mono text-gray-800 dark:text-white select-none">{{ row.total_quantity }}</td>
                <td class="py-2.5 text-right font-mono font-bold text-gray-800 dark:text-white select-none">{{ formatRupiah(row.total_revenue) }}</td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>

      <!-- Aksi Ekspor Data Kategori -->
      <div class="flex justify-end gap-2 select-none">
        <BaseButton variant="secondary" class="flex items-center gap-1.5 text-xs font-bold" @click="exportKategoriPdf">
          <DocumentArrowDownIcon class="w-3.5 h-3.5 text-rose-500/80" />
          <span>Ekspor PDF</span>
        </BaseButton>
        <BaseButton variant="secondary" class="flex items-center gap-1.5 text-xs font-bold" @click="exportKategoriExcel">
          <ArrowDownTrayIcon class="w-3.5 h-3.5 text-emerald-600/80" />
          <span>Ekspor Excel</span>
        </BaseButton>
      </div>
    </div>

    <!-- Tab Content: Percetakan -->
    <div v-else-if="activeTab === 'percetakan'" class="space-y-4 select-none">
      <div class="bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-lg p-6 text-center max-w-xl mx-auto">
        <PrinterIcon class="w-8 h-8 text-gray-400 dark:text-gray-500 mx-auto mb-2 opacity-40" />
        <p class="text-xs font-bold text-gray-400 dark:text-gray-500 uppercase tracking-wider">Total Pendapatan Percetakan (Status Terbayar)</p>
        <p class="text-2xl font-mono font-black text-gray-800 dark:text-white mt-1.5">{{ formatRupiah(store.serviceTypeReport.percetakan) }}</p>
        <p class="text-xs text-gray-500 dark:text-gray-400 mt-3 border-t border-gray-100 dark:border-gray-800 pt-3">
          Rincian dokumen antrian, proses cetak, dan pembaruan berkas lembar kerja dikelola mandiri via dashboard 
          <router-link to="/percetakan" class="text-indigo-600 dark:text-indigo-400 hover:underline font-bold inline-flex items-center gap-0.5 ml-0.5">
            Manajemen Percetakan &rarr;
          </router-link>
        </p>
      </div>
    </div>

    <!-- Tab Content: Jasa Digital PPOB -->
    <div v-else-if="activeTab === 'jasa'" class="space-y-4 select-none">
      <div class="bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-lg p-6 text-center max-w-xl mx-auto">
        <CpuChipIcon class="w-8 h-8 text-gray-400 dark:text-gray-500 mx-auto mb-2 opacity-40" />
        <p class="text-xs font-bold text-gray-400 dark:text-gray-500 uppercase tracking-wider">Total Pendapatan Sektor Jasa Digital</p>
        <p class="text-2xl font-mono font-black text-gray-800 dark:text-white mt-1.5">{{ formatRupiah(store.serviceTypeReport.jasa_digital) }}</p>
        <p class="text-xs text-gray-500 dark:text-gray-400 mt-3 border-t border-gray-100 dark:border-gray-800 pt-3">
          Pembagian laba bersih untuk transaksi Token PLN, Pulsa Seluler, Paket Data, dan Saldo E-Wallet dapat diperiksa secara mendetail di menu 
          <router-link to="/jasa" class="text-indigo-600 dark:text-indigo-400 hover:underline font-bold inline-flex items-center gap-0.5 ml-0.5">
            Layanan Jasa Digital &rarr;
          </router-link>
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
import { 
  CalendarDaysIcon, 
  PresentationChartLineIcon, 
  DocumentCheckIcon, 
  ExclamationTriangleIcon, 
  ShoppingBagIcon, 
  PrinterIcon, 
  CpuChipIcon, 
  DocumentArrowDownIcon, 
  ArrowDownTrayIcon,
  TagIcon 
} from "@heroicons/vue/24/outline";

// Tambahkan ini sementara
console.log({
  CalendarDaysIcon, 
  PresentationChartLineIcon, 
  DocumentCheckIcon, 
  ExclamationTriangleIcon, 
  ShoppingBagIcon, 
  PrinterIcon, 
  CpuChipIcon, 
  DocumentArrowDownIcon, 
  ArrowDownTrayIcon,
  TagIcon 
});

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