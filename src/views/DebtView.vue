<template>
  <div class="space-y-4">
    <!-- Toolbar -->
    <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-3 select-none">
      <div class="flex gap-1.5 border border-gray-200 dark:border-gray-800 p-1 rounded-lg bg-gray-50 dark:bg-gray-950">
        <button
          v-for="tab in filterTabs"
          :key="tab.value ?? 'all'"
          class="px-3 py-1.5 rounded-md text-xs font-bold whitespace-nowrap border transition-all"
          :class="activeFilter === tab.value
            ? 'bg-white border-gray-200 text-blue-600 shadow-none dark:bg-gray-900 dark:border-gray-800 dark:text-blue-400'
            : 'bg-transparent border-transparent text-gray-500 dark:text-gray-400 hover:text-gray-800 dark:hover:text-gray-200'"
          @click="setFilter(tab.value)"
        >
          {{ tab.label }}
        </button>
      </div>
      <BaseButton class="flex items-center gap-1.5" @click="openCreate">
        <PlusIcon class="w-4 h-4" />
        <span>Catat Hutang Baru</span>
      </BaseButton>
    </div>

    <!-- Ringkasan -->
    <div class="grid grid-cols-3 gap-3">
      <div class="card p-4 border border-gray-200 dark:border-gray-800">
        <p class="text-xs font-semibold text-gray-500 dark:text-gray-400 mb-1">Hutang Aktif</p>
        <p class="text-xl font-black text-gray-900 dark:text-white">{{ debtStore.summary.count }}</p>
      </div>
      <div class="card p-4 border border-gray-200 dark:border-gray-800">
        <p class="text-xs font-semibold text-gray-500 dark:text-gray-400 mb-1">Total Nominal</p>
        <p class="text-xl font-black text-gray-900 dark:text-white">{{ formatRupiah(debtStore.summary.total_amount) }}</p>
      </div>
      <div class="card p-4 border border-gray-200 dark:border-gray-800">
        <p class="text-xs font-semibold text-gray-500 dark:text-gray-400 mb-1">Total Sisa</p>
        <p class="text-xl font-black text-red-600 dark:text-red-400">{{ formatRupiah(debtStore.summary.total_remaining) }}</p>
      </div>
    </div>

    <!-- Tabel -->
    <BaseTable :columns="columns" :rows="paginatedRows">
      <template #cell-amount="{ row }">{{ formatRupiah(row.amount) }}</template>
      <template #cell-paid="{ row }">
        <span class="text-green-600 dark:text-green-400">{{ formatRupiah(row.paid) }}</span>
      </template>
      <template #cell-remaining="{ row }">
        <span class="font-bold" :class="row.remaining > 0 ? 'text-red-600 dark:text-red-400' : 'text-green-600 dark:text-green-400'">
          {{ formatRupiah(row.remaining) }}
        </span>
      </template>
      <template #cell-status="{ row }">
        <span
          class="px-2 py-0.5 rounded text-xs font-bold"
          :class="row.status === 'LUNAS'
            ? 'bg-green-50 text-green-600 dark:bg-green-900/20 dark:text-green-400'
            : 'bg-red-50 text-red-600 dark:bg-red-900/20 dark:text-red-400'"
        >{{ row.status }}</span>
      </template>
      <template #cell-due_date="{ row }">
        <span
          v-if="row.due_date"
          class="text-xs font-medium"
          :class="isOverdue(row) ? 'text-red-500 font-bold' : 'text-gray-500 dark:text-gray-400'"
        >
          {{ row.due_date }}
          <span v-if="isOverdue(row)" class="ml-1">⚠️</span>
        </span>
        <span v-else class="text-gray-400">-</span>
      </template>
      <template #actions="{ row }">
        <div class="flex justify-end gap-1 select-none">
          <button
            v-if="row.status === 'AKTIF'"
            class="flex items-center gap-1 px-2 py-1 text-xs font-semibold text-green-600 dark:text-green-400 hover:bg-green-50 dark:hover:bg-green-900/20 border border-transparent hover:border-green-200 dark:hover:border-green-800/50 rounded-md transition-colors"
            @click="openPayment(row)"
          >
            <BanknotesIcon class="w-3.5 h-3.5" />
            <span>Bayar</span>
          </button>
          <button
            class="flex items-center gap-1 px-2 py-1 text-xs font-semibold text-blue-600 dark:text-blue-400 hover:bg-blue-50 dark:hover:bg-blue-900/20 border border-transparent hover:border-blue-200 dark:hover:border-blue-800/50 rounded-md transition-colors"
            @click="openHistory(row)"
          >
            <ClockIcon class="w-3.5 h-3.5" />
            <span>Riwayat</span>
          </button>
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

    <BasePagination
      v-model="currentPage"
      :total="debtStore.debts.length"
      :per-page="perPage"
    />

    <p v-if="debtStore.loading" class="text-xs text-gray-400 px-1">Memuat data hutang...</p>

    <div
      v-if="!debtStore.loading && debtStore.debts.length === 0"
      class="bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-lg p-12 text-center text-gray-400 select-none"
    >
      <BanknotesIcon class="w-10 h-10 mx-auto mb-2 opacity-30" />
      <p class="text-sm font-bold text-gray-900 dark:text-white">Tidak ada data hutang</p>
      <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">Klik "Catat Hutang Baru" untuk mulai mencatat</p>
    </div>

    <!-- Modal Tambah Hutang -->
    <BaseModal v-model="showCreateModal" title="Catat Hutang Baru">
      <form class="space-y-4" @submit.prevent="submitCreate">
        <BaseInput v-model="createForm.name" label="Nama Pelanggan" required placeholder="Budi Santoso" />
        <BaseInput v-model="createForm.phone" label="Nomor HP" placeholder="08xxxxxxxxxx" />
        <BaseInput v-model="createForm.amount" type="number" step="500" label="Nominal Hutang" required placeholder="50000" />
        <BaseInput v-model="createForm.due_date" type="date" label="Jatuh Tempo (opsional)" />
        <div>
          <label class="block text-xs font-bold text-gray-700 dark:text-gray-300 uppercase tracking-wider mb-1.5">Keterangan</label>
          <textarea
            v-model="createForm.notes"
            class="w-full bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-md px-3 py-2 text-sm font-medium text-gray-900 dark:text-white focus:outline-none focus:border-blue-500 transition-colors resize-none"
            rows="2"
            placeholder="Beli apa, kapan, dll..."
          ></textarea>
        </div>
        <div class="flex justify-end gap-2 pt-3 border-t border-gray-100 dark:border-gray-800">
          <BaseButton type="button" variant="secondary" @click="showCreateModal = false">Batal</BaseButton>
          <BaseButton type="submit">Simpan</BaseButton>
        </div>
      </form>
    </BaseModal>

    <!-- Modal Catat Pembayaran -->
    <BaseModal v-model="showPaymentModal" :title="`Catat Pembayaran — ${paymentTarget?.name}`">
      <div class="space-y-4">
        <div class="bg-gray-50 dark:bg-gray-800 rounded-lg p-3 space-y-1 text-sm">
          <div class="flex justify-between">
            <span class="text-gray-500">Total Hutang</span>
            <span class="font-bold">{{ formatRupiah(paymentTarget?.amount) }}</span>
          </div>
          <div class="flex justify-between">
            <span class="text-gray-500">Sudah Dibayar</span>
            <span class="font-semibold text-green-600 dark:text-green-400">{{ formatRupiah(paymentTarget?.paid) }}</span>
          </div>
          <div class="flex justify-between border-t border-gray-200 dark:border-gray-700 pt-1">
            <span class="text-gray-500">Sisa Hutang</span>
            <span class="font-black text-red-600 dark:text-red-400">{{ formatRupiah(paymentTarget?.remaining) }}</span>
          </div>
        </div>

        <form class="space-y-3" @submit.prevent="submitPayment">
          <BaseInput
            v-model="paymentForm.amount"
            type="number"
            step="500"
            label="Jumlah Bayar"
            required
            :placeholder="`Maks. ${formatRupiah(paymentTarget?.remaining)}`"
          />
          <div>
            <label class="block text-xs font-bold text-gray-700 dark:text-gray-300 uppercase tracking-wider mb-1.5">Catatan</label>
            <textarea
              v-model="paymentForm.notes"
              class="w-full bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-md px-3 py-2 text-sm font-medium text-gray-900 dark:text-white focus:outline-none focus:border-blue-500 transition-colors resize-none"
              rows="2"
              placeholder="Catatan pembayaran..."
            ></textarea>
          </div>
          <div class="flex justify-end gap-2 pt-3 border-t border-gray-100 dark:border-gray-800">
            <BaseButton type="button" variant="secondary" @click="showPaymentModal = false">Batal</BaseButton>
            <BaseButton type="submit">Catat Pembayaran</BaseButton>
          </div>
        </form>
      </div>
    </BaseModal>

    <!-- Modal Riwayat Pembayaran -->
    <BaseModal v-model="showHistoryModal" :title="`Riwayat — ${historyTarget?.name}`">
      <div class="space-y-3">
        <div class="bg-gray-50 dark:bg-gray-800 rounded-lg p-3 space-y-1 text-sm">
          <div class="flex justify-between">
            <span class="text-gray-500">Total Hutang</span>
            <span class="font-bold">{{ formatRupiah(historyTarget?.amount) }}</span>
          </div>
          <div class="flex justify-between">
            <span class="text-gray-500">Sisa</span>
            <span class="font-black" :class="historyTarget?.remaining > 0 ? 'text-red-600 dark:text-red-400' : 'text-green-600 dark:text-green-400'">
              {{ formatRupiah(historyTarget?.remaining) }}
            </span>
          </div>
        </div>

        <p v-if="historyLoading" class="text-xs text-gray-400">Memuat riwayat...</p>
        <div v-else-if="paymentHistory.length === 0" class="text-center py-6 text-gray-400 text-sm">
          Belum ada riwayat pembayaran
        </div>
        <div v-else class="space-y-2">
          <div
            v-for="p in paymentHistory"
            :key="p.id"
            class="flex items-center justify-between bg-green-50 dark:bg-green-900/20 border border-green-100 dark:border-green-900/40 rounded-lg px-3 py-2"
          >
            <div>
              <p class="text-sm font-bold text-green-700 dark:text-green-300">{{ formatRupiah(p.amount) }}</p>
              <p class="text-xs text-gray-400">{{ formatDate(p.date) }}</p>
              <p v-if="p.notes" class="text-xs text-gray-500 mt-0.5">{{ p.notes }}</p>
            </div>
          </div>
        </div>
      </div>
    </BaseModal>

    <!-- Modal Konfirmasi Hapus -->
    <BaseModal v-model="showDeleteModal" title="Hapus Data Hutang?">
      <p class="text-sm text-gray-500 dark:text-gray-400 mb-4">
        Yakin ingin menghapus hutang <strong class="text-gray-900 dark:text-white">{{ deleteTarget?.name }}</strong>?
        Seluruh riwayat pembayaran juga akan terhapus.
      </p>
      <div class="flex gap-2">
        <BaseButton variant="secondary" class="flex-1" @click="showDeleteModal = false">Batal</BaseButton>
        <BaseButton variant="danger" class="flex-1" @click="doDelete">Hapus</BaseButton>
      </div>
    </BaseModal>
  </div>
</template>

<script setup>
import { ref, reactive, computed, onMounted } from "vue";
import { useDebtStore } from "../stores/debtStore";
import BaseButton from "../components/ui/BaseButton.vue";
import BaseInput from "../components/ui/BaseInput.vue";
import BaseModal from "../components/ui/BaseModal.vue";
import BaseTable from "../components/ui/BaseTable.vue";
import BasePagination from "../components/ui/BasePagination.vue";
import { PlusIcon, TrashIcon, BanknotesIcon, ClockIcon } from "@heroicons/vue/24/outline";

const debtStore = useDebtStore();

const activeFilter = ref(null);
const currentPage = ref(1);
const perPage = 20;

const filterTabs = [
  { value: null, label: "Semua" },
  { value: "AKTIF", label: "Aktif" },
  { value: "LUNAS", label: "Lunas" },
];

const columns = [
  { key: "name", label: "Nama Pelanggan" },
  { key: "phone", label: "Telepon" },
  { key: "amount", label: "Total Hutang" },
  { key: "paid", label: "Dibayar" },
  { key: "remaining", label: "Sisa" },
  { key: "due_date", label: "Jatuh Tempo" },
  { key: "status", label: "Status" },
];

const paginatedRows = computed(() => {
  const start = (currentPage.value - 1) * perPage;
  return debtStore.debts.slice(start, start + perPage);
});

// Modal state
const showCreateModal = ref(false);
const showPaymentModal = ref(false);
const showHistoryModal = ref(false);
const showDeleteModal = ref(false);

const paymentTarget = ref(null);
const historyTarget = ref(null);
const deleteTarget = ref(null);
const paymentHistory = ref([]);
const historyLoading = ref(false);

const createForm = reactive({
  name: "", phone: "", amount: 0, notes: "", due_date: "",
});

const paymentForm = reactive({
  amount: 0, notes: "",
});

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

function isOverdue(row) {
  if (!row.due_date || row.status === "LUNAS") return false;
  return new Date(row.due_date) < new Date();
}

function setFilter(val) {
  activeFilter.value = val;
  currentPage.value = 1;
  debtStore.fetchAll(val);
}

function openCreate() {
  Object.assign(createForm, { name: "", phone: "", amount: 0, notes: "", due_date: "" });
  showCreateModal.value = true;
}

async function submitCreate() {
  try {
    await debtStore.add({
      name: createForm.name,
      phone: createForm.phone || null,
      amount: Number(createForm.amount),
      notes: createForm.notes || null,
      due_date: createForm.due_date || null,
    });
    showCreateModal.value = false;
  } catch {
    // toast sudah ditampilkan store
  }
}

function openPayment(row) {
  paymentTarget.value = row;
  paymentForm.amount = row.remaining;
  paymentForm.notes = "";
  showPaymentModal.value = true;
}

async function submitPayment() {
  try {
    await debtStore.addPayment(
      paymentTarget.value.id,
      Number(paymentForm.amount),
      paymentForm.notes || null
    );
    showPaymentModal.value = false;
  } catch {
    // toast sudah ditampilkan store
  }
}

async function openHistory(row) {
  historyTarget.value = row;
  showHistoryModal.value = true;
  historyLoading.value = true;
  try {
    paymentHistory.value = await debtStore.getPayments(row.id);
  } catch {
    paymentHistory.value = [];
  } finally {
    historyLoading.value = false;
  }
}

function confirmDelete(row) {
  deleteTarget.value = row;
  showDeleteModal.value = true;
}

async function doDelete() {
  if (deleteTarget.value) {
    await debtStore.remove(deleteTarget.value.id);
  }
  showDeleteModal.value = false;
}

onMounted(async () => {
  await debtStore.fetchAll();
  await debtStore.fetchSummary();
});
</script>