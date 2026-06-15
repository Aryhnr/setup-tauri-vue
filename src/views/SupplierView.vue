<template>
  <div class="space-y-4">
    <!-- Toolbar -->
    <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-3 select-none">
      <div class="relative w-full sm:w-80">
        <input
          v-model="keyword"
          type="text"
          class="w-full bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-md pl-10 pr-4 py-2 text-sm font-medium text-gray-900 dark:text-white placeholder-gray-400 focus:outline-none focus:border-blue-500 dark:focus:border-blue-400 transition-colors"
          placeholder="Cari nama supplier..."
          @input="search"
        />
        <div class="absolute left-3 top-2.5 flex items-center pointer-events-none">
          <MagnifyingGlassIcon class="w-4 h-4 text-gray-400" />
        </div>
      </div>
      <BaseButton class="flex items-center gap-1.5" @click="openCreate">
        <PlusIcon class="w-4 h-4" />
        <span>Tambah Supplier</span>
      </BaseButton>
    </div>

    <!-- Tabel -->
    <BaseTable :columns="columns" :rows="paginatedRows">
      <template #cell-phone="{ row }">{{ row.phone || "-" }}</template>
      <template #cell-address="{ row }">
        <span class="text-sm text-gray-500 dark:text-gray-400 line-clamp-1">{{ row.address || "-" }}</span>
      </template>
      <template #cell-notes="{ row }">
        <span class="text-sm text-gray-500 dark:text-gray-400 line-clamp-1">{{ row.notes || "-" }}</span>
      </template>
      <template #actions="{ row }">
        <div class="flex justify-end gap-1 select-none">
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
      :total="supplierStore.suppliers.length"
      :per-page="perPage"
    />

    <!-- State Status Footer -->
    <div class="flex flex-col gap-1 text-xs font-medium select-none px-1">
      <p v-if="supplierStore.loading" class="text-gray-400">Memuat basis data supplier...</p>
      <p v-if="supplierStore.error" class="text-red-500 font-bold">{{ supplierStore.error }}</p>
    </div>

    <!-- Empty State -->
    <div
      v-if="!supplierStore.loading && supplierStore.suppliers.length === 0"
      class="bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-lg p-12 text-center text-gray-400 select-none"
    >
      <TruckIcon class="w-10 h-10 mx-auto mb-2 opacity-30" />
      <p class="text-sm font-bold text-gray-900 dark:text-white">Belum ada data supplier</p>
      <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">Klik tombol "+ Tambah Supplier" untuk mulai</p>
    </div>

    <!-- Modal Tambah / Edit -->
    <BaseModal v-model="showModal" :title="isEdit ? 'Edit Supplier' : 'Tambah Supplier'">
      <form class="space-y-4" @submit.prevent="submit">
        <BaseInput v-model="form.name" label="Nama Supplier" required placeholder="CV. Sumber Makmur" />
        <BaseInput v-model="form.phone" label="Nomor Telepon" placeholder="08xxxxxxxxxx" />
        <div>
          <label class="block text-xs font-bold text-gray-700 dark:text-gray-300 uppercase tracking-wider mb-1.5">Alamat</label>
          <textarea
            v-model="form.address"
            class="w-full bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-md px-3 py-2 text-sm font-medium text-gray-900 dark:text-white focus:outline-none focus:border-blue-500 dark:focus:border-blue-400 transition-colors resize-none"
            rows="2"
            placeholder="Jl. Raya No. 1, Surabaya"
          ></textarea>
        </div>
        <div>
          <label class="block text-xs font-bold text-gray-700 dark:text-gray-300 uppercase tracking-wider mb-1.5">Catatan</label>
          <textarea
            v-model="form.notes"
            class="w-full bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-md px-3 py-2 text-sm font-medium text-gray-900 dark:text-white focus:outline-none focus:border-blue-500 dark:focus:border-blue-400 transition-colors resize-none"
            rows="2"
            placeholder="Catatan tambahan..."
          ></textarea>
        </div>
        <div class="flex justify-end gap-2 pt-3 border-t border-gray-100 dark:border-gray-800">
          <BaseButton type="button" variant="secondary" @click="showModal = false">Batal</BaseButton>
          <BaseButton type="submit">{{ isEdit ? "Simpan" : "Tambah" }}</BaseButton>
        </div>
      </form>
    </BaseModal>

    <!-- Modal Konfirmasi Hapus -->
    <BaseModal v-model="showDeleteModal" title="Hapus Supplier">
      <div class="select-none">
        <p class="text-sm text-gray-500 dark:text-gray-400 mb-5">
          Yakin ingin menghapus supplier <span class="font-bold text-gray-900 dark:text-white">[{{ deleteTarget?.name }}]</span>?
          Data produk yang terhubung tidak akan ikut terhapus.
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
import { useSupplierStore } from "../stores/supplierStore";
import BaseButton from "../components/ui/BaseButton.vue";
import BaseInput from "../components/ui/BaseInput.vue";
import BaseModal from "../components/ui/BaseModal.vue";
import BaseTable from "../components/ui/BaseTable.vue";
import BasePagination from "../components/ui/BasePagination.vue";
import {
  MagnifyingGlassIcon,
  PlusIcon,
  PencilSquareIcon,
  TrashIcon,
  TruckIcon,
} from "@heroicons/vue/24/outline";

const supplierStore = useSupplierStore();

const keyword = ref("");
const showModal = ref(false);
const showDeleteModal = ref(false);
const isEdit = ref(false);
const editId = ref(null);
const deleteTarget = ref(null);

const currentPage = ref(1);
const perPage = 20;

const columns = [
  { key: "name", label: "Nama Supplier" },
  { key: "phone", label: "Telepon" },
  { key: "address", label: "Alamat" },
  { key: "notes", label: "Catatan" },
];

const emptyForm = () => ({
  name: "",
  phone: "",
  address: "",
  notes: "",
});

const form = reactive(emptyForm());

const paginatedRows = computed(() => {
  const start = (currentPage.value - 1) * perPage;
  return supplierStore.suppliers.slice(start, start + perPage);
});

function search() {
  currentPage.value = 1;
  supplierStore.fetchAll(keyword.value);
}

function openCreate() {
  Object.assign(form, emptyForm());
  isEdit.value = false;
  editId.value = null;
  showModal.value = true;
}

function openEdit(row) {
  Object.assign(form, {
    name: row.name,
    phone: row.phone || "",
    address: row.address || "",
    notes: row.notes || "",
  });
  isEdit.value = true;
  editId.value = row.id;
  showModal.value = true;
}

async function submit() {
  try {
    if (isEdit.value) {
      await supplierStore.update(editId.value, form);
    } else {
      await supplierStore.add(form);
    }
    showModal.value = false;
  } catch (err) {
    console.error("Gagal menyimpan data:", err);
  }
}

function confirmDelete(row) {
  deleteTarget.value = row;
  showDeleteModal.value = true;
}

async function doDelete() {
  if (deleteTarget.value) {
    await supplierStore.remove(deleteTarget.value.id);
  }
  showDeleteModal.value = false;
}

onMounted(() => supplierStore.fetchAll());
</script>