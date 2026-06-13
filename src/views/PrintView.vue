<template>
  <div class="space-y-4">
    <!-- Toolbar -->
    <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-3">
      <div class="flex gap-2 overflow-x-auto">
        <button
          v-for="tab in statusTabs"
          :key="tab.value || 'all'"
          class="px-3 py-1.5 rounded-lg text-sm font-medium whitespace-nowrap transition-colors"
          :class="activeStatus === tab.value
            ? 'bg-blue-600 text-white'
            : 'bg-gray-100 text-gray-600 dark:bg-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-600'"
          @click="setStatus(tab.value)"
        >
          {{ tab.label }}
        </button>
      </div>
      <BaseButton @click="openCreate">+ Order Baru</BaseButton>
    </div>

    <!-- Tabel order -->
    <BaseTable :columns="columns" :rows="store.orders">
      <template #cell-status="{ row }"><BadgeStatus :status="row.status" /></template>
      <template #cell-service_type="{ row }">{{ SERVICE_LABELS[row.service_type] || row.service_type }}</template>
      <template #cell-color_mode="{ row }">{{ row.color_mode === "BERWARNA" ? "Berwarna" : "Hitam Putih" }}</template>
      <template #cell-total_price="{ row }">{{ formatRupiah(row.total_price) }}</template>
      <template #cell-created_at="{ row }">{{ formatDate(row.created_at) }}</template>

      <template #actions="{ row }">
        <div class="flex justify-end gap-2 items-center">
          <select
            class="text-xs border border-gray-200 dark:border-gray-600 rounded bg-white dark:bg-gray-700 px-1.5 py-1"
            :value="row.status"
            @change="store.updateStatus(row.id, $event.target.value)"
          >
            <option v-for="s in STATUS_OPTIONS" :key="s" :value="s">{{ STATUS_LABELS[s] }}</option>
          </select>
          <button class="text-blue-600 hover:underline text-sm" @click="openEdit(row)">Edit</button>
          <button class="text-red-500 hover:underline text-sm" @click="confirmDelete(row)">Hapus</button>
        </div>
      </template>
    </BaseTable>

    <p v-if="store.loading" class="text-sm text-gray-400">Memuat data...</p>
    <p v-if="store.error" class="text-sm text-red-500">{{ store.error }}</p>

    <!-- Modal Tambah / Edit -->
    <BaseModal v-model="showModal" :title="isEdit ? 'Edit Order Percetakan' : 'Order Percetakan Baru'">
      <form class="space-y-3" @submit.prevent="submit">
        <div>
          <label class="label">Jenis Layanan</label>
          <select v-model="form.service_type" class="input">
            <option value="PRINT_DOC">Print Dokumen</option>
            <option value="PRINT_FOTO">Print Foto</option>
            <option value="FOTOCOPY">Fotocopy</option>
          </select>
        </div>

        <div class="grid grid-cols-2 gap-3">
          <div>
            <label class="label">Ukuran Kertas</label>
            <select v-model="form.paper_size" class="input">
              <option value="A4">A4</option>
              <option value="F4">F4</option>
              <option value="A3">A3</option>
              <option value="A5">A5</option>
            </select>
          </div>
          <div>
            <label class="label">Mode Warna</label>
            <select v-model="form.color_mode" class="input">
              <option value="HITAM_PUTIH">Hitam Putih</option>
              <option value="BERWARNA">Berwarna</option>
            </select>
          </div>
        </div>

        <div class="grid grid-cols-3 gap-3">
          <BaseInput v-model="form.pages" type="number" label="Jumlah Halaman" />
          <BaseInput v-model="form.copies" type="number" label="Jumlah Rangkap" />
          <BaseInput v-model="form.price_per_page" type="number" step="100" label="Harga / Lembar" />
        </div>

        <div class="card p-3 bg-blue-50 dark:bg-blue-900/20 border-blue-100 dark:border-blue-800">
          <div class="flex items-center justify-between text-sm">
            <span class="text-gray-500">Total Harga</span>
            <span class="font-bold text-lg">{{ formatRupiah(computedTotal) }}</span>
          </div>
        </div>

        <div class="flex justify-end gap-2 pt-2">
          <BaseButton type="button" variant="secondary" @click="showModal = false">Batal</BaseButton>
          <BaseButton type="submit">{{ isEdit ? "Simpan" : "Buat Order" }}</BaseButton>
        </div>
      </form>
    </BaseModal>

    <!-- Modal konfirmasi hapus -->
    <BaseModal v-model="showDeleteModal" title="Hapus Order">
      <p class="text-sm text-gray-500 mb-4">
        Yakin ingin menghapus order <strong>{{ deleteTarget?.order_no }}</strong>?
      </p>
      <div class="flex justify-end gap-2">
        <BaseButton variant="secondary" @click="showDeleteModal = false">Batal</BaseButton>
        <BaseButton variant="danger" @click="doDelete">Hapus</BaseButton>
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