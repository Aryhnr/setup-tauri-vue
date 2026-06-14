<template>
  <div class="space-y-4">
    <!-- Ringkasan Hari Ini -->
    <div class="grid grid-cols-1 sm:grid-cols-3 gap-4 select-none">
      <div class="bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-lg p-4 flex items-center justify-between">
        <div>
          <p class="text-[10px] font-black text-gray-400 dark:text-gray-500 uppercase tracking-wider">Total Penjualan</p>
          <p class="text-xl font-mono font-black text-gray-900 dark:text-white mt-1">{{ formatRupiah(store.summaryToday.total_sell) }}</p>
        </div>
        <div class="p-2 bg-gray-50 dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-md text-gray-400">
          <BanknotesIcon class="w-5 h-5" />
        </div>
      </div>

      <div class="bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-lg p-4 flex items-center justify-between">
        <div>
          <p class="text-[10px] font-black text-gray-400 dark:text-gray-500 uppercase tracking-wider">Total Keuntungan</p>
          <p class="text-xl font-mono font-black text-green-600 dark:text-green-400 mt-1">
            {{ formatRupiah(store.summaryToday.total_profit) }}
          </p>
        </div>
        <div class="p-2 bg-green-50 dark:bg-green-950 border border-green-100 dark:border-green-900 rounded-md text-green-600 dark:text-green-400">
          <ArrowTrendingUpIcon class="w-5 h-5" />
        </div>
      </div>

      <div class="bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-lg p-4 flex items-center justify-between">
        <div>
          <p class="text-[10px] font-black text-gray-400 dark:text-gray-500 uppercase tracking-wider">Jumlah Transaksi</p>
          <p class="text-xl font-mono font-black text-gray-900 dark:text-white mt-1">{{ store.summaryToday.count }}</p>
        </div>
        <div class="p-2 bg-gray-50 dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-md text-gray-400">
          <DocumentCheckIcon class="w-5 h-5" />
        </div>
      </div>
    </div>

    <!-- Toolbar -->
    <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-3 select-none">
      <!-- Segmented Type Tabs Filter -->
      <div class="flex gap-1.5 border border-gray-200 dark:border-gray-800 p-1 rounded-lg bg-gray-50 dark:bg-gray-950 overflow-x-auto invisible-scrollbar">
        <button
          v-for="tab in typeTabs"
          :key="tab.value || 'all'"
          class="px-3 py-1.5 rounded-md text-xs font-bold whitespace-nowrap border transition-all"
          :class="activeType === tab.value
            ? 'bg-white border-gray-200 text-blue-600 dark:bg-gray-900 dark:border-gray-800 dark:text-blue-400 font-extrabold'
            : 'bg-transparent border-transparent text-gray-500 dark:text-gray-400 hover:text-gray-800 dark:hover:text-gray-200'"
          @click="setType(tab.value)"
        >
          {{ tab.label }}
        </button>
      </div>
      <BaseButton class="flex items-center gap-1.5 self-end sm:self-auto" @click="openCreate">
        <PlusIcon class="w-4 h-4" />
        <span>Tambah Transaksi</span>
      </BaseButton>
    </div>

    <!-- Tabel -->
    <BaseTable :columns="columns" :rows="store.services">
      <template #cell-service_type="{ row }">{{ TYPE_LABELS[row.service_type] || row.service_type }}</template>
      <template #cell-sell_price="{ row }">{{ formatRupiah(row.sell_price) }}</template>
      <template #cell-buy_price="{ row }">{{ formatRupiah(row.buy_price) }}</template>
      <template #cell-profit="{ row }">
        <span class="text-green-600 dark:text-green-400 font-bold">{{ formatRupiah(row.profit) }}</span>
      </template>
      <template #cell-transaction_date="{ row }">{{ formatDate(row.transaction_date) }}</template>
      <template #cell-customer_info="{ row }">{{ row.customer_info || "-" }}</template>

      <template #actions="{ row }">
        <div class="flex justify-end select-none">
          <button 
            class="flex items-center gap-1 px-2 py-1 text-xs font-semibold text-red-500 dark:text-red-400 hover:bg-red-50 dark:hover:bg-red-900/20 border border-transparent hover:border-red-200 dark:hover:border-red-800/50 rounded-md transition-colors" 
            @click="confirmDelete(row)"
          >
            <TrashIcon class="w-3.5 h-3.5" />
            <span>Hapus</span>
          </button>
        </div>
      </template>
    </BaseTable>

    <!-- State Status Footer -->
    <div class="flex flex-col gap-1 text-xs font-medium select-none px-1">
      <p v-if="store.loading" class="text-gray-400">Sinkronisasi data transaksi...</p>
      <p v-if="store.error" class="text-red-500 font-bold">{{ store.error }}</p>
    </div>

    <!-- Modal Tambah -->
    <BaseModal v-model="showModal" title="Tambah Transaksi Jasa">
      <form class="space-y-4" @submit.prevent="submit">
        <div>
          <label class="block text-xs font-bold text-gray-700 dark:text-gray-300 uppercase tracking-wider mb-1.5">Jenis Layanan</label>
          <select 
            v-model="form.service_type" 
            class="w-full bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-md px-3 py-2 text-sm font-medium text-gray-900 dark:text-white focus:outline-none focus:border-blue-500 dark:focus:border-blue-400 transition-colors"
          >
            <option value="TOKEN_PLN">Token Listrik PLN</option>
            <option value="TOPUP">Top Up E-Wallet</option>
            <option value="PULSA">Pulsa</option>
            <option value="PAKET_DATA">Paket Data</option>
          </select>
        </div>

        <BaseInput v-model="form.description" label="Keterangan" placeholder="Token 50rb / Dana 50rb / dll" />
        <BaseInput
          v-model="form.customer_info"
          :label="form.service_type === 'TOKEN_PLN' ? 'No. Meter' : 'No. HP / Tujuan'"
          placeholder="(opsional)"
        />

        <div class="grid grid-cols-2 gap-4">
          <BaseInput v-model="form.buy_price" type="number" step="500" label="Harga Modal" />
          <BaseInput v-model="form.sell_price" type="number" step="500" label="Harga Jual" />
        </div>

        <!-- Panel Kalkulasi Keuntungan -->
        <div class="bg-green-50/50 dark:bg-green-950/20 border border-green-200 dark:border-green-900/50 rounded-lg p-4 select-none">
          <div class="flex items-center justify-between">
            <span class="text-xs font-bold text-green-700 dark:text-green-400 uppercase tracking-wider">Estimasi Profit Jasa</span>
            <span class="font-mono font-black text-xl text-green-600 dark:text-green-400">{{ formatRupiah(computedProfit) }}</span>
          </div>
        </div>

        <div class="flex justify-end gap-2 pt-3 border-t border-gray-100 dark:border-gray-800">
          <BaseButton type="button" variant="secondary" @click="showModal = false">Batal</BaseButton>
          <BaseButton type="submit">Simpan</BaseButton>
        </div>
      </form>
    </BaseModal>

    <!-- Modal Konfirmasi Hapus -->
    <BaseModal v-model="showDeleteModal" title="Hapus Transaksi">
      <div class="select-none">
        <p class="text-sm text-gray-500 dark:text-gray-400 mb-5">
          Yakin ingin menghapus dokumen transaksi ini? Tindakan ini akan memperbarui akumulasi laba harian sistem.
        </p>
        <div class="flex justify-end gap-2">
          <BaseButton variant="secondary" @click="showDeleteModal = false">Batal</BaseButton>
          <BaseButton variant="danger" @click="doDelete">Hapus</BaseButton>
        </div>
      </div>
    </BaseModal>
  </div>
</template>

<script setup>
import { ref, reactive, computed, onMounted } from "vue";
import { useServiceStore } from "../stores/serviceStore";
import BaseButton from "../components/ui/BaseButton.vue";
import BaseInput from "../components/ui/BaseInput.vue";
import BaseModal from "../components/ui/BaseModal.vue";
import BaseTable from "../components/ui/BaseTable.vue";
import { 
  BanknotesIcon, 
  ArrowTrendingUpIcon, 
  DocumentCheckIcon, 
  PlusIcon, 
  TrashIcon 
} from "@heroicons/vue/24/outline";

const store = useServiceStore();

const activeType = ref(null);
const showModal = ref(false);
const showDeleteModal = ref(false);
const deleteTarget = ref(null);

const TYPE_LABELS = {
  TOKEN_PLN: "Token PLN",
  TOPUP: "Top Up E-Wallet",
  PULSA: "Pulsa",
  PAKET_DATA: "Paket Data",
};

const typeTabs = [
  { value: null, label: "Semua" },
  { value: "TOKEN_PLN", label: "Token PLN" },
  { value: "TOPUP", label: "Top Up" },
  { value: "PULSA", label: "Pulsa" },
  { value: "PAKET_DATA", label: "Paket Data" },
];

const columns = [
  { key: "service_type", label: "Jenis" },
  { key: "description", label: "Keterangan" },
  { key: "customer_info", label: "No. Meter/HP" },
  { key: "buy_price", label: "Modal" },
  { key: "sell_price", label: "Harga Jual" },
  { key: "profit", label: "Untung" },
  { key: "transaction_date", label: "Waktu" },
];

const emptyForm = () => ({
  service_type: "TOKEN_PLN",
  description: "",
  customer_info: "",
  buy_price: 0,
  sell_price: 0,
});

const form = reactive(emptyForm());

const computedProfit = computed(() => (Number(form.sell_price) || 0) - (Number(form.buy_price) || 0));

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

function setType(type) {
  activeType.value = type;
  store.fetchAll(type);
}

function openCreate() {
  Object.assign(form, emptyForm());
  showModal.value = true;
}

async function submit() {
  await store.add(form);
  showModal.value = false;
}

function confirmDelete(row) {
  deleteTarget.value = row;
  showDeleteModal.value = true;
}

async function doDelete() {
  if (deleteTarget.value) {
    await store.remove(deleteTarget.value.id);
  }
  showDeleteModal.value = false;
}

onMounted(() => {
  store.fetchAll();
  store.fetchSummaryToday();
});
</script>