<template>
  <div class="space-y-4">
    <!-- Filter bar -->
    <div class="flex flex-wrap gap-3 items-end">
      <div>
        <label class="label">Dari Tanggal</label>
        <input v-model="filter.startDate" type="date" class="input" />
      </div>
      <div>
        <label class="label">Sampai Tanggal</label>
        <input v-model="filter.endDate" type="date" class="input" />
      </div>
      <div>
        <label class="label">Tipe</label>
        <select v-model="filter.type" class="input">
          <option value="">Semua</option>
          <option value="PENJUALAN">Penjualan</option>
          <option value="PERCETAKAN">Percetakan</option>
          <option value="JASA">Jasa</option>
        </select>
      </div>
      <BaseButton @click="fetchData">Tampilkan</BaseButton>
    </div>

    <!-- Ringkasan -->
    <div class="grid grid-cols-2 sm:grid-cols-3 gap-3">
      <div class="card p-4 border border-gray-200 dark:border-gray-800">
        <p class="text-xs font-semibold text-gray-500 dark:text-gray-400 mb-1">Total Transaksi</p>
        <p class="text-xl font-black text-gray-900 dark:text-white">{{ transactions.length }}</p>
      </div>
      <div class="card p-4 border border-gray-200 dark:border-gray-800">
        <p class="text-xs font-semibold text-gray-500 dark:text-gray-400 mb-1">Total Omset</p>
        <p class="text-xl font-black text-gray-900 dark:text-white">{{ formatRupiah(totalRevenue) }}</p>
      </div>
      <div class="card p-4 border border-gray-200 dark:border-gray-800">
        <p class="text-xs font-semibold text-gray-500 dark:text-gray-400 mb-1">Rata-rata / Transaksi</p>
        <p class="text-xl font-black text-gray-900 dark:text-white">{{ formatRupiah(avgRevenue) }}</p>
      </div>
    </div>

    <!-- Tabel -->
    <p v-if="loading" class="text-xs font-medium text-gray-400 px-1 select-none">Memuat data transaksi...</p>

    <BaseTable :columns="columns" :rows="paginatedRows" @row-click="openDetail">
      <template #cell-invoice_no="{ row }">
        <span class="font-bold text-blue-600 dark:text-blue-400">{{ row.invoice_no }}</span>
      </template>
      <template #cell-transaction_date="{ row }">
        <span class="text-xs text-gray-500">{{ formatDate(row.transaction_date) }}</span>
      </template>
      <template #cell-transaction_type="{ row }">
        <span
          class="px-2 py-0.5 rounded text-xs font-bold"
          :class="{
            'bg-blue-50 text-blue-600 dark:bg-blue-900/20 dark:text-blue-400': row.transaction_type === 'PENJUALAN',
            'bg-purple-50 text-purple-600 dark:bg-purple-900/20 dark:text-purple-400': row.transaction_type === 'PERCETAKAN',
            'bg-orange-50 text-orange-600 dark:bg-orange-900/20 dark:text-orange-400': row.transaction_type === 'JASA',
          }"
        >{{ row.transaction_type }}</span>
      </template>
      <template #cell-total_amount="{ row }">
        <span class="font-semibold">{{ formatRupiah(row.total_amount) }}</span>
      </template>
      <template #cell-paid_amount="{ row }">
        <span class="text-sm text-gray-500">{{ formatRupiah(row.paid_amount) }}</span>
      </template>
      <template #cell-change_amount="{ row }">
        <span class="text-sm text-green-600 dark:text-green-400">{{ formatRupiah(row.change_amount) }}</span>
      </template>
      <template #actions="{ row }">
        <div class="flex justify-end" @click.stop>
          <button
            class="text-xs text-red-500 hover:text-red-700 font-semibold px-2 py-1 hover:bg-red-50 dark:hover:bg-red-900/20 rounded transition-colors"
            @click="confirmVoid(row)"
          >
            Void
          </button>
        </div>
      </template>
    </BaseTable>

    <BasePagination
      v-model="currentPage"
      :total="transactions.length"
      :per-page="perPage"
    />

    <div
      v-if="!loading && transactions.length === 0"
      class="bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-lg p-12 text-center text-gray-400 select-none"
    >
      <ClipboardDocumentListIcon class="w-10 h-10 mx-auto mb-2 opacity-30" />
      <p class="text-sm font-bold text-gray-900 dark:text-white">Tidak ada transaksi</p>
      <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">Coba ubah filter tanggal atau tipe transaksi</p>
    </div>

    <!-- Modal Detail Transaksi -->
    <BaseModal v-model="showDetailModal" :title="`Detail — ${selectedTx?.invoice_no}`">
      <div class="space-y-4">
        <div class="grid grid-cols-2 gap-2 text-sm">
          <div>
            <p class="text-xs text-gray-400">Tanggal</p>
            <p class="font-semibold text-gray-900 dark:text-white">{{ formatDate(selectedTx?.transaction_date) }}</p>
          </div>
          <div>
            <p class="text-xs text-gray-400">Tipe</p>
            <p class="font-semibold text-gray-900 dark:text-white">{{ selectedTx?.transaction_type }}</p>
          </div>
        </div>

        <div v-if="loadingDetail" class="text-xs text-gray-400 py-4 text-center">Memuat detail...</div>

        <div v-else class="border border-gray-100 dark:border-gray-800 rounded-lg overflow-hidden">
          <table class="w-full text-sm">
            <thead class="bg-gray-50 dark:bg-gray-800">
              <tr>
                <th class="text-left px-3 py-2 text-xs font-bold text-gray-500 dark:text-gray-400">Item</th>
                <th class="text-center px-3 py-2 text-xs font-bold text-gray-500 dark:text-gray-400">Qty</th>
                <th class="text-right px-3 py-2 text-xs font-bold text-gray-500 dark:text-gray-400">Harga</th>
                <th class="text-right px-3 py-2 text-xs font-bold text-gray-500 dark:text-gray-400">Subtotal</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="item in detailItems" :key="item.id" class="border-t border-gray-100 dark:border-gray-800">
                <td class="px-3 py-2 font-medium text-gray-900 dark:text-white">{{ item.item_name }}</td>
                <td class="px-3 py-2 text-center text-gray-500">{{ item.quantity }}</td>
                <td class="px-3 py-2 text-right text-gray-500">{{ formatRupiah(item.unit_price) }}</td>
                <td class="px-3 py-2 text-right font-semibold text-gray-900 dark:text-white">{{ formatRupiah(item.subtotal) }}</td>
              </tr>
            </tbody>
          </table>
        </div>

        <div class="bg-gray-50 dark:bg-gray-800 rounded-lg p-3 space-y-1.5 text-sm">
          <div class="flex justify-between font-black text-gray-900 dark:text-white">
            <span>Total</span>
            <span>{{ formatRupiah(selectedTx?.total_amount) }}</span>
          </div>
          <div class="flex justify-between text-gray-500 text-xs">
            <span>Bayar</span>
            <span>{{ formatRupiah(selectedTx?.paid_amount) }}</span>
          </div>
          <div class="flex justify-between text-xs">
            <span class="text-gray-500">Kembali</span>
            <span class="font-semibold text-green-600 dark:text-green-400">{{ formatRupiah(selectedTx?.change_amount) }}</span>
          </div>
        </div>

        <div class="flex gap-2">
          <BaseButton variant="secondary" class="flex-1" :disabled="printing" @click="printDetail">
            <PrinterIcon class="w-4 h-4 mr-1.5 inline" />
            {{ printing ? "Mencetak..." : "Cetak Ulang" }}
          </BaseButton>
          <BaseButton variant="danger" class="flex-1" @click="confirmVoidFromDetail">
            Void Transaksi
          </BaseButton>
        </div>
      </div>
    </BaseModal>

    <!-- Modal Konfirmasi Void -->
    <BaseModal v-model="showVoidModal" title="Batalkan Transaksi?">
      <div class="space-y-4">
        <div class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-900/40 rounded-lg p-4">
          <p class="text-sm font-semibold text-red-700 dark:text-red-400">⚠️ Peringatan!</p>
          <p class="text-sm text-red-600 dark:text-red-300 mt-1">
            Transaksi <strong>{{ voidTarget?.invoice_no }}</strong> akan dibatalkan.
            Stok produk akan dikembalikan secara otomatis.
          </p>
        </div>
        <p class="text-sm text-gray-500 dark:text-gray-400">Tindakan ini tidak bisa dibatalkan.</p>
        <div class="flex gap-2">
          <BaseButton variant="secondary" class="flex-1" @click="showVoidModal = false">Batal</BaseButton>
          <BaseButton variant="danger" class="flex-1" :disabled="voidLoading" @click="doVoid">
            {{ voidLoading ? "Memproses..." : "Ya, Batalkan Transaksi" }}
          </BaseButton>
        </div>
      </div>
    </BaseModal>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useToast } from "../composables/useToast";
import { useSettingStore } from "../stores/settingStore";
import { printHtml } from "../composables/usePrint";
import BaseButton from "../components/ui/BaseButton.vue";
import BaseModal from "../components/ui/BaseModal.vue";
import BaseTable from "../components/ui/BaseTable.vue";
import BasePagination from "../components/ui/BasePagination.vue";
import { PrinterIcon, ClipboardDocumentListIcon } from "@heroicons/vue/24/outline";

const { success, error: toastError } = useToast();
const settingStore = useSettingStore();

const today = new Date().toISOString().slice(0, 10);
const filter = ref({
  startDate: today.slice(0, 7) + "-01",
  endDate: today,
  type: "",
});

const transactions = ref([]);
const loading = ref(false);
const printing = ref(false);
const loadingDetail = ref(false);

const currentPage = ref(1);
const perPage = 20;

const columns = [
  { key: "invoice_no", label: "No. Invoice" },
  { key: "transaction_date", label: "Tanggal" },
  { key: "transaction_type", label: "Tipe" },
  { key: "total_amount", label: "Total" },
  { key: "paid_amount", label: "Bayar" },
  { key: "change_amount", label: "Kembali" },
];

const paginatedRows = computed(() => {
  const start = (currentPage.value - 1) * perPage;
  return transactions.value.slice(start, start + perPage);
});

const totalRevenue = computed(() => transactions.value.reduce((s, t) => s + t.total_amount, 0));
const avgRevenue = computed(() => transactions.value.length ? totalRevenue.value / transactions.value.length : 0);

const showDetailModal = ref(false);
const selectedTx = ref(null);
const detailItems = ref([]);

const showVoidModal = ref(false);
const voidTarget = ref(null);
const voidLoading = ref(false);

function formatRupiah(value) {
  return new Intl.NumberFormat("id-ID", {
    style: "currency", currency: "IDR", minimumFractionDigits: 0,
  }).format(value || 0);
}

function formatDate(dateStr) {
  if (!dateStr) return "-";
  const d = new Date(dateStr.replace(" ", "T") + "Z");
  return d.toLocaleString("id-ID", {
    day: "numeric", month: "short", year: "numeric",
    hour: "2-digit", minute: "2-digit",
  });
}

async function fetchData() {
  loading.value = true;
  currentPage.value = 1;
  try {
    transactions.value = await invoke("get_transactions", {
      startDate: filter.value.startDate,
      endDate: filter.value.endDate,
      transactionType: filter.value.type || null,
    });
  } catch (err) {
    toastError("Gagal memuat transaksi: " + err);
  } finally {
    loading.value = false;
  }
}

async function openDetail(tx) {
  selectedTx.value = tx;
  detailItems.value = [];
  showDetailModal.value = true;
  loadingDetail.value = true;
  try {
    detailItems.value = await invoke("get_transaction_detail", { transactionId: tx.id });
  } catch {
    detailItems.value = [];
    toastError("Gagal memuat detail transaksi");
  } finally {
    loadingDetail.value = false;
  }
}

function confirmVoid(tx) {
  voidTarget.value = tx;
  showVoidModal.value = true;
}

function confirmVoidFromDetail() {
  voidTarget.value = selectedTx.value;
  showDetailModal.value = false;
  showVoidModal.value = true;
}

async function doVoid() {
  if (!voidTarget.value) return;
  voidLoading.value = true;
  try {
    await invoke("void_transaction", { transactionId: voidTarget.value.id });
    success(`Transaksi ${voidTarget.value.invoice_no} berhasil dibatalkan`);
    showVoidModal.value = false;
    voidTarget.value = null;
    await fetchData();
  } catch (err) {
    toastError("Gagal membatalkan transaksi: " + err);
  } finally {
    voidLoading.value = false;
  }
}

async function printDetail() {
  printing.value = true;
  try {
    const s = settingStore;
    const itemRows = detailItems.value.map((i) =>
      `<tr>
        <td>${i.item_name}</td>
        <td style="text-align:center">${i.quantity}</td>
        <td style="text-align:right">${formatRupiah(i.unit_price)}</td>
        <td style="text-align:right">${formatRupiah(i.subtotal)}</td>
      </tr>`
    ).join("");

    const html = `<!DOCTYPE html><html><head><meta charset="utf-8">
    <title>Struk ${selectedTx.value?.invoice_no}</title>
    <style>
      *{margin:0;padding:0;box-sizing:border-box}
      body{font-family:monospace;font-size:12px;padding:16px;max-width:280px}
      table{width:100%;border-collapse:collapse}
      th,td{padding:3px 4px;font-size:11px}
      th{border-bottom:1px dashed #ccc;text-align:left}
      .center{text-align:center}.bold{font-weight:700}.big{font-size:14px}
      .gray{color:#666}.divider{border-top:1px dashed #ccc;margin:8px 0}
      .row{display:flex;justify-content:space-between}
      @page{size:80mm auto;margin:0}
      @media print{body{padding:8px}}
    </style></head><body>
    <div class="center bold big">${s.storeName || "Kasir"}</div>
    ${s.storeAddress ? `<div class="center gray">${s.storeAddress}</div>` : ""}
    ${s.storePhone ? `<div class="center gray">Telp: ${s.storePhone}</div>` : ""}
    <div class="divider"></div>
    <div>No: <b>${selectedTx.value?.invoice_no}</b></div>
    <div class="gray">${formatDate(selectedTx.value?.transaction_date)}</div>
    <div class="divider"></div>
    <table>
      <thead><tr><th>Item</th><th>Qty</th><th>Harga</th><th>Sub</th></tr></thead>
      <tbody>${itemRows}</tbody>
    </table>
    <div class="divider"></div>
    <div class="row"><span class="bold">TOTAL</span><span class="bold">${formatRupiah(selectedTx.value?.total_amount)}</span></div>
    <div class="row gray"><span>Bayar</span><span>${formatRupiah(selectedTx.value?.paid_amount)}</span></div>
    <div class="row gray"><span>Kembali</span><span>${formatRupiah(selectedTx.value?.change_amount)}</span></div>
    <div class="divider"></div>
    <div class="center gray">Terima kasih!</div>
    </body></html>`;

    await printHtml(html);
    success("Struk berhasil dicetak!");
  } catch (err) {
    console.error("printDetail error:", err);
    toastError("Gagal mencetak struk");
  } finally {
    printing.value = false;
  }
}

onMounted(fetchData);
</script>