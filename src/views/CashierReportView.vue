<template>
  <div class="space-y-6 max-w-3xl">

    <!-- Rekap Hari Ini -->
    <div class="card p-6 border border-gray-200 dark:border-gray-800 space-y-5">
      <div class="flex items-center justify-between">
        <h2 class="text-base font-bold text-gray-900 dark:text-white">Tutup Kasir Hari Ini</h2>
        <span class="text-xs text-gray-400">{{ todayLabel }}</span>
      </div>

      <!-- Ringkasan dari DB -->
      <div class="grid grid-cols-2 sm:grid-cols-4 gap-3">
        <div class="bg-blue-50 dark:bg-blue-900/20 border border-blue-100 dark:border-blue-900/40 rounded-lg p-3">
          <p class="text-xs font-semibold text-blue-500 dark:text-blue-400 mb-1">Penjualan</p>
          <p class="text-sm font-black text-blue-700 dark:text-blue-300">{{ formatRupiah(daily.total_penjualan) }}</p>
        </div>
        <div class="bg-purple-50 dark:bg-purple-900/20 border border-purple-100 dark:border-purple-900/40 rounded-lg p-3">
          <p class="text-xs font-semibold text-purple-500 dark:text-purple-400 mb-1">Percetakan</p>
          <p class="text-sm font-black text-purple-700 dark:text-purple-300">{{ formatRupiah(daily.total_percetakan) }}</p>
        </div>
        <div class="bg-orange-50 dark:bg-orange-900/20 border border-orange-100 dark:border-orange-900/40 rounded-lg p-3">
          <p class="text-xs font-semibold text-orange-500 dark:text-orange-400 mb-1">Jasa</p>
          <p class="text-sm font-black text-orange-700 dark:text-orange-300">{{ formatRupiah(daily.total_jasa) }}</p>
        </div>
        <div class="bg-green-50 dark:bg-green-900/20 border border-green-100 dark:border-green-900/40 rounded-lg p-3">
          <p class="text-xs font-semibold text-green-500 dark:text-green-400 mb-1">Total Pemasukan</p>
          <p class="text-sm font-black text-green-700 dark:text-green-300">{{ formatRupiah(daily.total_pemasukan) }}</p>
        </div>
      </div>

      <!-- Form input kasir -->
      <div class="space-y-4">
        <BaseInput
          v-model="form.modal_awal"
          type="number"
          step="1000"
          label="Modal Awal (uang di laci saat buka toko)"
          placeholder="500000"
        />

        <!-- Kalkulasi otomatis -->
        <div class="bg-gray-50 dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg p-4 space-y-2 text-sm">
          <div class="flex justify-between">
            <span class="text-gray-500">Modal Awal</span>
            <span class="font-semibold">{{ formatRupiah(Number(form.modal_awal) || 0) }}</span>
          </div>
          <div class="flex justify-between">
            <span class="text-gray-500">+ Total Pemasukan</span>
            <span class="font-semibold text-green-600 dark:text-green-400">{{ formatRupiah(daily.total_pemasukan) }}</span>
          </div>
          <div class="border-t border-gray-200 dark:border-gray-700 pt-2 flex justify-between font-bold">
            <span class="text-gray-700 dark:text-gray-300">= Total Seharusnya di Laci</span>
            <span class="text-gray-900 dark:text-white">{{ formatRupiah(totalSeharusnya) }}</span>
          </div>
        </div>

        <BaseInput
          v-model="form.uang_aktual"
          type="number"
          step="1000"
          label="Uang Aktual di Laci (hitung manual)"
          placeholder="0"
        />

        <!-- Selisih -->
        <div
          class="rounded-lg p-4 border text-sm"
          :class="selisih === 0
            ? 'bg-green-50 border-green-200 dark:bg-green-900/20 dark:border-green-900/40'
            : selisih > 0
              ? 'bg-blue-50 border-blue-200 dark:bg-blue-900/20 dark:border-blue-900/40'
              : 'bg-red-50 border-red-200 dark:bg-red-900/20 dark:border-red-900/40'"
        >
          <div class="flex justify-between items-center">
            <span class="font-semibold"
              :class="selisih === 0
                ? 'text-green-700 dark:text-green-300'
                : selisih > 0
                  ? 'text-blue-700 dark:text-blue-300'
                  : 'text-red-700 dark:text-red-300'"
            >
              {{ selisih === 0 ? '✓ Pas / Sesuai' : selisih > 0 ? '↑ Lebih' : '↓ Kurang' }}
            </span>
            <span class="text-xl font-black"
              :class="selisih === 0
                ? 'text-green-700 dark:text-green-300'
                : selisih > 0
                  ? 'text-blue-700 dark:text-blue-300'
                  : 'text-red-700 dark:text-red-300'"
            >
              {{ selisih >= 0 ? '' : '-' }}{{ formatRupiah(Math.abs(selisih)) }}
            </span>
          </div>
          <p class="text-xs mt-1 opacity-70"
            :class="selisih === 0
              ? 'text-green-600 dark:text-green-400'
              : selisih > 0
                ? 'text-blue-600 dark:text-blue-400'
                : 'text-red-600 dark:text-red-400'"
          >
            {{ selisih === 0
              ? 'Uang di laci sesuai dengan yang seharusnya.'
              : selisih > 0
                ? 'Uang di laci lebih dari yang seharusnya.'
                : 'Uang di laci kurang dari yang seharusnya.' }}
          </p>
        </div>

        <div>
          <label class="block text-xs font-bold text-gray-700 dark:text-gray-300 uppercase tracking-wider mb-1.5">Catatan</label>
          <textarea
            v-model="form.catatan"
            class="w-full bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-md px-3 py-2 text-sm font-medium text-gray-900 dark:text-white focus:outline-none focus:border-blue-500 transition-colors resize-none"
            rows="2"
            placeholder="Catatan tutup kasir hari ini..."
          ></textarea>
        </div>

        <div class="flex gap-2">
          <BaseButton
            variant="secondary"
            class="flex items-center gap-1.5"
            :disabled="printing"
            @click="printReport"
          >
            <PrinterIcon class="w-4 h-4" />
            {{ printing ? "Mencetak..." : "Cetak Rekap" }}
          </BaseButton>
          <BaseButton class="flex-1" :disabled="saving" @click="saveReport">
            {{ saving ? "Menyimpan..." : "Simpan Rekap Kasir" }}
          </BaseButton>
        </div>

        <p v-if="saveError" class="text-sm text-red-500">{{ saveError }}</p>
      </div>
    </div>

    <!-- Riwayat Rekap -->
    <div class="card p-6 border border-gray-200 dark:border-gray-800 space-y-4">
      <h2 class="text-base font-bold text-gray-900 dark:text-white">Riwayat Rekap Kasir</h2>

      <p v-if="loadingHistory" class="text-xs text-gray-400">Memuat riwayat...</p>

      <div v-else-if="history.length === 0" class="text-center py-8 text-gray-400">
        <ClipboardDocumentListIcon class="w-8 h-8 mx-auto mb-2 opacity-30" />
        <p class="text-sm">Belum ada rekap tersimpan</p>
      </div>

      <BaseTable v-else :columns="historyColumns" :rows="paginatedHistory">
        <template #cell-tanggal="{ row }">
          <span class="font-semibold text-gray-900 dark:text-white">{{ row.tanggal }}</span>
        </template>
        <template #cell-modal_awal="{ row }">{{ formatRupiah(row.modal_awal) }}</template>
        <template #cell-total_pemasukan="{ row }">
          <span class="text-green-600 dark:text-green-400 font-semibold">{{ formatRupiah(row.total_pemasukan) }}</span>
        </template>
        <template #cell-uang_aktual="{ row }">{{ formatRupiah(row.uang_aktual) }}</template>
        <template #cell-selisih="{ row }">
          <span
            class="font-bold"
            :class="row.selisih === 0
              ? 'text-green-600 dark:text-green-400'
              : row.selisih > 0
                ? 'text-blue-600 dark:text-blue-400'
                : 'text-red-600 dark:text-red-400'"
          >
            {{ row.selisih >= 0 ? '+' : '' }}{{ formatRupiah(row.selisih) }}
          </span>
        </template>
      </BaseTable>

      <BasePagination
        v-model="currentPage"
        :total="history.length"
        :per-page="perPage"
      />
    </div>
  </div>
</template>

<script setup>
import { ref, reactive, computed, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useToast } from "../composables/useToast";
import { useSettingStore } from "../stores/settingStore";
import { printHtml } from "../composables/usePrint";
import BaseButton from "../components/ui/BaseButton.vue";
import BaseInput from "../components/ui/BaseInput.vue";
import BaseTable from "../components/ui/BaseTable.vue";
import BasePagination from "../components/ui/BasePagination.vue";
import { PrinterIcon, ClipboardDocumentListIcon } from "@heroicons/vue/24/outline";

const { success, error: toastError, info } = useToast();
const settingStore = useSettingStore();

const today = new Date();
const todayLabel = today.toLocaleDateString("id-ID", {
  weekday: "long", day: "numeric", month: "long", year: "numeric",
});

const daily = ref({
  total_penjualan: 0,
  total_percetakan: 0,
  total_jasa: 0,
  total_pemasukan: 0,
  jumlah_transaksi: 0,
});

const form = reactive({
  modal_awal: 0,
  uang_aktual: 0,
  catatan: "",
});

const saving = ref(false);
const printing = ref(false);
const saveError = ref("");
const history = ref([]);
const loadingHistory = ref(false);
const currentPage = ref(1);
const perPage = 10;

const totalSeharusnya = computed(() =>
  (Number(form.modal_awal) || 0) + daily.value.total_pemasukan
);

const selisih = computed(() =>
  (Number(form.uang_aktual) || 0) - totalSeharusnya.value
);

const historyColumns = [
  { key: "tanggal", label: "Tanggal" },
  { key: "modal_awal", label: "Modal Awal" },
  { key: "total_pemasukan", label: "Pemasukan" },
  { key: "uang_aktual", label: "Aktual" },
  { key: "selisih", label: "Selisih" },
];

const paginatedHistory = computed(() => {
  const start = (currentPage.value - 1) * perPage;
  return history.value.slice(start, start + perPage);
});

function formatRupiah(value) {
  return new Intl.NumberFormat("id-ID", {
    style: "currency", currency: "IDR", minimumFractionDigits: 0,
  }).format(value || 0);
}

async function fetchDaily() {
  try {
    daily.value = await invoke("get_daily_summary");
  } catch (err) {
    console.error("fetchDaily error:", err);
  }
}

async function fetchHistory() {
  loadingHistory.value = true;
  try {
    history.value = await invoke("get_cashier_reports", { limit: 60 });
  } catch (err) {
    console.error("fetchHistory error:", err);
  } finally {
    loadingHistory.value = false;
  }
}

async function saveReport() {
  saving.value = true;
  saveError.value = "";
  try {
    await invoke("save_cashier_report", {
      payload: {
        modal_awal: Number(form.modal_awal) || 0,
        uang_aktual: Number(form.uang_aktual) || 0,
        catatan: form.catatan || null,
      },
    });
    success("Rekap kasir berhasil disimpan!");
    await fetchHistory();
  } catch (err) {
    saveError.value = err?.toString() ?? "Gagal menyimpan";
    toastError("Gagal menyimpan rekap: " + saveError.value);
  } finally {
    saving.value = false;
  }
}

async function printReport() {
  printing.value = true;
  try {
    const s = settingStore;
    const selisihVal = selisih.value;
    const selisihClass = selisihVal === 0 ? "pas" : selisihVal > 0 ? "lebih" : "kurang";
    const selisihLabel = selisihVal === 0 ? "✓ Pas / Sesuai" : selisihVal > 0 ? "↑ Lebih" : "↓ Kurang";

    const html = `<!DOCTYPE html><html><head><meta charset="utf-8">
    <title>Rekap Kasir ${todayLabel}</title>
    <style>
      *{margin:0;padding:0;box-sizing:border-box}
      body{font-family:monospace;font-size:12px;padding:20px;max-width:320px}
      .center{text-align:center}.bold{font-weight:700}.big{font-size:15px}
      .gray{color:#666}.divider{border-top:1px dashed #ccc;margin:10px 0}
      .row{display:flex;justify-content:space-between;margin-bottom:4px}
      .selisih{font-size:16px;font-weight:900}
      .lebih{color:#2563eb}.kurang{color:#dc2626}.pas{color:#16a34a}
      @media print{body{padding:0}}
    </style></head><body>
    <div class="center bold big">${s.storeName || "Kasir"}</div>
    ${s.storeAddress ? `<div class="center gray">${s.storeAddress}</div>` : ""}
    ${s.storePhone ? `<div class="center gray">Telp: ${s.storePhone}</div>` : ""}
    <div class="divider"></div>
    <div class="center bold">REKAP TUTUP KASIR</div>
    <div class="center gray">${todayLabel}</div>
    <div class="divider"></div>
    <div class="row"><span>Modal Awal</span><span>${formatRupiah(Number(form.modal_awal) || 0)}</span></div>
    <div class="divider"></div>
    <div class="row"><span>Penjualan Barang</span><span>${formatRupiah(daily.value.total_penjualan)}</span></div>
    <div class="row"><span>Percetakan</span><span>${formatRupiah(daily.value.total_percetakan)}</span></div>
    <div class="row"><span>Jasa Digital</span><span>${formatRupiah(daily.value.total_jasa)}</span></div>
    <div class="row bold"><span>Total Pemasukan</span><span>${formatRupiah(daily.value.total_pemasukan)}</span></div>
    <div class="divider"></div>
    <div class="row bold"><span>Total Seharusnya</span><span>${formatRupiah(totalSeharusnya.value)}</span></div>
    <div class="row"><span>Uang Aktual di Laci</span><span>${formatRupiah(Number(form.uang_aktual) || 0)}</span></div>
    <div class="divider"></div>
    <div class="row">
      <span class="bold ${selisihClass}">${selisihLabel}</span>
      <span class="selisih ${selisihClass}">
        ${selisihVal >= 0 ? "+" : ""}${formatRupiah(selisihVal)}
      </span>
    </div>
    ${form.catatan ? `<div class="divider"></div><div class="gray">Catatan: ${form.catatan}</div>` : ""}
    <div class="divider"></div>
    <div class="center gray">Dicetak: ${new Date().toLocaleString("id-ID")}</div>
    </body></html>`;

    await printHtml(html);
    success("Rekap berhasil dicetak!");
  } catch (err) {
    console.error("printReport error:", err);
    toastError("Gagal mencetak rekap");
  } finally {
    printing.value = false;
  }
}

onMounted(async () => {
  await fetchDaily();
  await fetchHistory();
});
</script>