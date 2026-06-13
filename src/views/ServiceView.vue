<template>
  <div class="space-y-4">
    <!-- Ringkasan hari ini -->
    <div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
      <div class="card p-4">
        <p class="text-sm text-gray-400">Total Penjualan Hari Ini</p>
        <p class="text-2xl font-bold mt-1">{{ formatRupiah(store.summaryToday.total_sell) }}</p>
      </div>
      <div class="card p-4">
        <p class="text-sm text-gray-400">Total Keuntungan Hari Ini</p>
        <p class="text-2xl font-bold mt-1 text-green-600 dark:text-green-400">
          {{ formatRupiah(store.summaryToday.total_profit) }}
        </p>
      </div>
      <div class="card p-4">
        <p class="text-sm text-gray-400">Jumlah Transaksi Hari Ini</p>
        <p class="text-2xl font-bold mt-1">{{ store.summaryToday.count }}</p>
      </div>
    </div>

    <!-- Toolbar -->
    <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-3">
      <div class="flex gap-2 overflow-x-auto">
        <button
          v-for="tab in typeTabs"
          :key="tab.value || 'all'"
          class="px-3 py-1.5 rounded-lg text-sm font-medium whitespace-nowrap transition-colors"
          :class="activeType === tab.value
            ? 'bg-blue-600 text-white'
            : 'bg-gray-100 text-gray-600 dark:bg-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-600'"
          @click="setType(tab.value)"
        >
          {{ tab.label }}
        </button>
      </div>
      <BaseButton @click="openCreate">+ Tambah Transaksi</BaseButton>
    </div>

    <!-- Tabel -->
    <BaseTable :columns="columns" :rows="store.services">
      <template #cell-service_type="{ row }">{{ TYPE_LABELS[row.service_type] || row.service_type }}</template>
      <template #cell-sell_price="{ row }">{{ formatRupiah(row.sell_price) }}</template>
      <template #cell-buy_price="{ row }">{{ formatRupiah(row.buy_price) }}</template>
      <template #cell-profit="{ row }">
        <span class="text-green-600 dark:text-green-400 font-medium">{{ formatRupiah(row.profit) }}</span>
      </template>
      <template #cell-transaction_date="{ row }">{{ formatDate(row.transaction_date) }}</template>
      <template #cell-customer_info="{ row }">{{ row.customer_info || "-" }}</template>

      <template #actions="{ row }">
        <button class="text-red-500 hover:underline text-sm" @click="confirmDelete(row)">Hapus</button>
      </template>
    </BaseTable>

    <p v-if="store.loading" class="text-sm text-gray-400">Memuat data...</p>
    <p v-if="store.error" class="text-sm text-red-500">{{ store.error }}</p>

    <!-- Modal Tambah -->
    <BaseModal v-model="showModal" title="Tambah Transaksi Jasa">
      <form class="space-y-3" @submit.prevent="submit">
        <div>
          <label class="label">Jenis Layanan</label>
          <select v-model="form.service_type" class="input">
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

        <div class="grid grid-cols-2 gap-3">
          <BaseInput v-model="form.buy_price" type="number" step="500" label="Harga Modal" />
          <BaseInput v-model="form.sell_price" type="number" step="500" label="Harga Jual" />
        </div>

        <div class="card p-3 bg-green-50 dark:bg-green-900/20 border-green-100 dark:border-green-800">
          <div class="flex items-center justify-between text-sm">
            <span class="text-gray-500">Keuntungan</span>
            <span class="font-bold text-lg text-green-600 dark:text-green-400">{{ formatRupiah(computedProfit) }}</span>
          </div>
        </div>

        <div class="flex justify-end gap-2 pt-2">
          <BaseButton type="button" variant="secondary" @click="showModal = false">Batal</BaseButton>
          <BaseButton type="submit">Simpan</BaseButton>
        </div>
      </form>
    </BaseModal>

    <!-- Modal konfirmasi hapus -->
    <BaseModal v-model="showDeleteModal" title="Hapus Transaksi">
      <p class="text-sm text-gray-500 mb-4">Yakin ingin menghapus transaksi ini?</p>
      <div class="flex justify-end gap-2">
        <BaseButton variant="secondary" @click="showDeleteModal = false">Batal</BaseButton>
        <BaseButton variant="danger" @click="doDelete">Hapus</BaseButton>
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