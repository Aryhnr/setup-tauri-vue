<template>
  <div class="space-y-4">
    <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-3 select-none">
      <div class="flex gap-1.5 border border-gray-200 dark:border-gray-800 p-1 rounded-lg bg-gray-50 dark:bg-gray-950 overflow-x-auto invisible-scrollbar">
        <button
          v-for="tab in statusTabs"
          :key="tab.value || 'all'"
          class="px-3 py-1.5 rounded-md text-xs font-bold whitespace-nowrap border transition-all"
          :class="activeStatus === tab.value
            ? 'bg-white border-gray-200 text-blue-600 shadow-none dark:bg-gray-900 dark:border-gray-800 dark:text-blue-400 font-extrabold'
            : 'bg-transparent border-transparent text-gray-500 dark:text-gray-400 hover:text-gray-800 dark:hover:text-gray-200'"
          @click="setStatus(tab.value)"
        >
          {{ tab.label }}
        </button>
      </div>
      <BaseButton class="flex items-center gap-1.5 self-end sm:self-auto" @click="openCreate">
        <PlusIcon class="w-4 h-4" />
        <span>Order Baru</span>
      </BaseButton>
    </div>

    <BaseTable :columns="columns" :rows="paginatedRows">
      <template #cell-status="{ row }"><BadgeStatus :status="row.status" /></template>
      <template #cell-service_type="{ row }">{{ SERVICE_LABELS[row.service_type] || row.service_type }}</template>
      <template #cell-color_mode="{ row }">{{ row.color_mode === "BERWARNA" ? "Berwarna" : "Hitam Putih" }}</template>
      <template #cell-total_price="{ row }">{{ formatRupiah(row.total_price) }}</template>
      <template #cell-created_at="{ row }">{{ formatDate(row.created_at) }}</template>

      <template #actions="{ row }">
        <div class="flex justify-end gap-1.5 items-center select-none">
          <select
            class="text-xs bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-md px-2 py-1 font-bold text-gray-700 dark:text-gray-300 focus:outline-none focus:border-blue-500 dark:focus:border-blue-400 transition-colors cursor-pointer"
            :value="row.status"
            @change="store.updateStatus(row.id, $event.target.value)"
          >
            <option v-for="s in STATUS_OPTIONS" :key="s" :value="s">{{ STATUS_LABELS[s] }}</option>
          </select>
          
          <button 
            class="flex items-center gap-1 px-2 py-1 text-xs font-semibold text-blue-600 dark:text-blue-400 hover:bg-blue-50 dark:hover:bg-blue-900/20 border border-transparent hover:border-blue-200 dark:hover:border-blue-800/50 rounded-md transition-colors" 
            @click="openEdit(row)"
          >
            <PencilSquareIcon class="w-3.5 h-3.5" />
            <span>Edit</span>
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
      :total="store.orders.length"
      :per-page="perPage"
    />

    <div class="flex flex-col gap-1 text-xs font-medium select-none px-1">
      <p v-if="store.loading" class="text-gray-400">Syncing database order...</p>
      <p v-if="store.error" class="text-red-500 font-bold">{{ store.error }}</p>
    </div>

    <BaseModal v-model="showModal" :title="isEdit ? 'Edit Order Percetakan' : 'Order Percetakan Baru'">
      <form class="space-y-4" @submit.prevent="submit">
        <div>
          <label class="block text-xs font-bold text-gray-700 dark:text-gray-300 uppercase tracking-wider mb-1.5">Jenis Layanan</label>
          <select 
            v-model="form.service_type" 
            class="w-full bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-md px-3 py-2 text-sm font-medium text-gray-900 dark:text-white focus:outline-none focus:border-blue-500 dark:focus:border-blue-400 transition-colors"
          >
            <option value="PRINT_DOC">Print Dokumen</option>
            <option value="PRINT_FOTO">Print Foto</option>
            <option value="FOTOCOPY">Fotocopy</option>
          </select>
        </div>

        <div class="grid grid-cols-2 gap-4">
          <div>
            <label class="block text-xs font-bold text-gray-700 dark:text-gray-300 uppercase tracking-wider mb-1.5">Ukuran Kertas</label>
            <select 
              v-model="form.paper_size" 
              class="w-full bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-md px-3 py-2 text-sm font-medium text-gray-900 dark:text-white focus:outline-none focus:border-blue-500 dark:focus:border-blue-400 transition-colors"
            >
              <option value="A4">A4</option>
              <option value="F4">F4</option>
              <option value="A3">A3</option>
              <option value="A5">A5</option>
            </select>
          </div>
          <div>
            <label class="block text-xs font-bold text-gray-700 dark:text-gray-300 uppercase tracking-wider mb-1.5">Mode Warna</label>
            <select 
              v-model="form.color_mode" 
              class="w-full bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-md px-3 py-2 text-sm font-medium text-gray-900 dark:text-white focus:outline-none focus:border-blue-500 dark:focus:border-blue-400 transition-colors"
            >
              <option value="HITAM_PUTIH">Hitam Putih</option>
              <option value="BERWARNA">Berwarna</option>
            </select>
          </div>
        </div>

        <div class="grid grid-cols-3 gap-4">
          <BaseInput v-model="form.pages" type="number" label="Jumlah Halaman" />
          <BaseInput v-model="form.copies" type="number" label="Jumlah Rangkap" />
          <BaseInput v-model="form.price_per_page" type="number" step="100" label="Harga / Lembar" />
        </div>

        <div class="bg-gray-50 dark:bg-gray-950 border border-gray-200 dark:border-gray-800 rounded-lg p-4 select-none">
          <div class="flex items-center justify-between">
            <span class="text-xs font-bold text-gray-500 dark:text-gray-400 uppercase tracking-wider">Total Harga Klien</span>
            <span class="font-mono font-black text-xl text-gray-900 dark:text-white">{{ formatRupiah(computedTotal) }}</span>
          </div>
        </div>

        <div class="flex justify-end gap-2 pt-3 border-t border-gray-100 dark:border-gray-800">
          <BaseButton type="button" variant="secondary" @click="showModal = false">Batal</BaseButton>
          <BaseButton type="submit">{{ isEdit ? "Simpan" : "Buat Order" }}</BaseButton>
        </div>
      </form>
    </BaseModal>

    <BaseModal v-model="showDeleteModal" title="Hapus Order">
      <div class="select-none">
        <p class="text-sm text-gray-500 dark:text-gray-400 mb-5">
          Yakin ingin menghapus dokumen order <span class="font-bold text-gray-900 dark:text-white">[{{ deleteTarget?.order_no }}]</span>?
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
import { usePrintOrderStore } from "../stores/printOrderStore";
import BaseButton from "../components/ui/BaseButton.vue";
import BaseInput from "../components/ui/BaseInput.vue";
import BaseModal from "../components/ui/BaseModal.vue";
import BaseTable from "../components/ui/BaseTable.vue";
import BadgeStatus from "../components/ui/BadgeStatus.vue";
// Tambah di bagian import
import BasePagination from "../components/ui/BasePagination.vue";
import { PlusIcon, PencilSquareIcon, TrashIcon } from "@heroicons/vue/24/outline";

const store = usePrintOrderStore();

const activeStatus = ref(null);
const showModal = ref(false);
const showDeleteModal = ref(false);
const isEdit = ref(false);
const editId = ref(null);
const deleteTarget = ref(null);

const STATUS_OPTIONS = ["ANTRIAN", "DIPROSES", "SELESAI", "DIBAYAR"];
const STATUS_LABELS = {
  ANTRIAN: "Antrian",
  DIPROSES: "Diproses",
  SELESAI: "Selesai",
  DIBAYAR: "Dibayar",
};
const SERVICE_LABELS = {
  PRINT_DOC: "Print Dokumen",
  PRINT_FOTO: "Print Foto",
  FOTOCOPY: "Fotocopy",
};

const statusTabs = [
  { value: null, label: "Semua" },
  { value: "ANTRIAN", label: "Antrian" },
  { value: "DIPROSES", label: "Diproses" },
  { value: "SELESAI", label: "Selesai" },
  { value: "DIBAYAR", label: "Dibayar" },
];

const columns = [
  { key: "order_no", label: "No. Order" },
  { key: "service_type", label: "Jenis" },
  { key: "paper_size", label: "Ukuran" },
  { key: "color_mode", label: "Warna" },
  { key: "pages", label: "Halaman" },
  { key: "copies", label: "Rangkap" },
  { key: "total_price", label: "Total" },
  { key: "status", label: "Status" },
  { key: "created_at", label: "Waktu" },
];

const emptyForm = () => ({
  service_type: "PRINT_DOC",
  paper_size: "A4",
  color_mode: "HITAM_PUTIH",
  pages: 1,
  copies: 1,
  price_per_page: 500,
});

const form = reactive(emptyForm());

const computedTotal = computed(() => {
  return (Number(form.pages) || 0) * (Number(form.copies) || 0) * (Number(form.price_per_page) || 0);
});

const currentPage = ref(1);
const perPage = 20;

// Sesuaikan nama array datanya (movements / suppliers / orders / services)
const paginatedRows = computed(() => {
  const start = (currentPage.value - 1) * perPage;
  return store.orders.slice(start, start + perPage);
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

function setStatus(status) {
  activeStatus.value = status;
  store.fetchAll(status);
}

function openCreate() {
  Object.assign(form, emptyForm());
  isEdit.value = false;
  editId.value = null;
  showModal.value = true;
}

function openEdit(row) {
  Object.assign(form, {
    service_type: row.service_type,
    paper_size: row.paper_size || "A4",
    color_mode: row.color_mode || "HITAM_PUTIH",
    pages: row.pages,
    copies: row.copies,
    price_per_page: row.price_per_page,
  });
  isEdit.value = true;
  editId.value = row.id;
  showModal.value = true;
}

async function submit() {
  const payload = { ...form, total_price: computedTotal.value };
  if (isEdit.value) {
    const current = store.orders.find((o) => o.id === editId.value);
    await store.update(editId.value, { ...payload, status: current?.status || "ANTRIAN" });
  } else {
    await store.add(payload);
  }
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
});
</script>