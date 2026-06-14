<template>
  <div class="space-y-4">
    <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-3 select-none">
      <div class="relative w-full sm:w-80">
        <input
          v-model="keyword"
          type="text"
          class="w-full bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-md pl-10 pr-4 py-2 text-sm font-medium text-gray-900 dark:text-white placeholder-gray-400 focus:outline-none focus:border-blue-500 dark:focus:border-blue-400 transition-colors"
          placeholder="Cari nama produk atau scan barcode..."
          @keyup.enter="search"
        />
        <div class="absolute left-3 top-2.5 flex items-center pointer-events-none">
          <MagnifyingGlassIcon class="w-4 h-4 text-gray-400" />
        </div>
      </div>
      <BaseButton class="flex items-center gap-1.5" @click="openCreate">
        <PlusIcon class="w-4 h-4" />
        <span>Tambah Produk</span>
      </BaseButton>
    </div>

    <div
      v-if="productStore.lowStockProducts.length"
      class="flex items-start gap-3 p-4 bg-amber-50 dark:bg-amber-900/20 border border-amber-200 dark:border-amber-900/50 rounded-lg select-none"
    >
      <ExclamationTriangleIcon class="w-5 h-5 text-amber-600 dark:text-amber-400 flex-shrink-0 mt-0.5" />
      <div>
        <p class="text-sm font-bold text-amber-800 dark:text-amber-300">
          {{ productStore.lowStockProducts.length }} Produk Stok Menipis
        </p>
        <p class="text-xs font-medium text-amber-700/80 dark:text-amber-400/80 mt-0.5">
          {{ productStore.lowStockProducts.map((p) => p.name).join(", ") }}
        </p>
      </div>
    </div>

    <BaseTable :columns="columns" :rows="productStore.products">
      <template #cell-stock="{ row }">
        <span
          class="px-2.5 py-1 rounded-md text-xs font-bold border"
          :class="row.stock <= row.min_stock
            ? 'bg-red-50 text-red-600 border-red-200 dark:bg-red-900/20 dark:text-red-400 dark:border-red-900/50'
            : 'bg-green-50 text-green-600 border-green-200 dark:bg-green-900/20 dark:text-green-400 dark:border-green-900/50'"
        >
          {{ row.stock }} {{ row.unit }}
        </span>
      </template>
      <template #cell-buy_price="{ row }">{{ formatRupiah(row.buy_price) }}</template>
      <template #cell-sell_price="{ row }">{{ formatRupiah(row.sell_price) }}</template>
      <template #cell-category_name="{ row }">{{ row.category_name || "-" }}</template>
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

    <div class="flex flex-col gap-1 text-xs font-medium select-none px-1">
      <p v-if="productStore.loading" class="text-gray-400">Memuat data sistem...</p>
      <p v-if="productStore.error" class="text-red-500 font-bold">{{ productStore.error }}</p>
    </div>

    <BaseModal v-model="showModal" :title="isEdit ? 'Edit Produk' : 'Tambah Produk'">
      <form class="space-y-4" @submit.prevent="submit">
        <BaseInput v-model="form.name" label="Nama Produk" required placeholder="Indomie Goreng" />

        <!-- Barcode: ketik manual, scan langsung, atau generate -->
        <div>
          <label class="block text-xs font-bold text-gray-700 dark:text-gray-300 uppercase tracking-wider mb-1.5">
            Barcode
          </label>
          <div class="flex gap-2">
            <input
              v-model="form.barcode"
              type="text"
              data-barcode-input="true"
              class="flex-1 bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-md px-3 py-2 text-sm font-medium text-gray-900 dark:text-white focus:outline-none focus:border-blue-500 dark:focus:border-blue-400 transition-colors"
              placeholder="Ketik manual atau scan barcode..."
            />
            <button
              type="button"
              class="flex items-center gap-1.5 px-3 py-2 text-xs font-bold bg-gray-100 dark:bg-gray-800 border border-gray-200 dark:border-gray-700 text-gray-600 dark:text-gray-300 rounded-md hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors whitespace-nowrap"
              @click="generateBarcode"
            >
              <ArrowPathIcon class="w-3.5 h-3.5" />
              Generate
            </button>
          </div>
          <p class="text-xs text-gray-400 mt-1">Ketik manual, scan langsung ke field ini, atau klik Generate untuk buat otomatis.</p>
        </div>

        <div>
          <label class="block text-xs font-bold text-gray-700 dark:text-gray-300 uppercase tracking-wider mb-1.5">Kategori</label>
          <select
            v-model="form.category_id"
            class="w-full bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-md px-3 py-2 text-sm font-medium text-gray-900 dark:text-white focus:outline-none focus:border-blue-500 dark:focus:border-blue-400 transition-colors"
          >
            <option :value="null">- Pilih kategori -</option>
            <option v-for="c in categoryStore.categories" :key="c.id" :value="c.id">{{ c.name }}</option>
          </select>
        </div>

        <div class="grid grid-cols-2 gap-4">
          <BaseInput v-model="form.buy_price" type="number" step="100" label="Harga Beli" />
          <BaseInput v-model="form.sell_price" type="number" step="100" label="Harga Jual" />
        </div>

        <div class="grid grid-cols-3 gap-4">
          <BaseInput v-model="form.stock" type="number" label="Stok" />
          <BaseInput v-model="form.min_stock" type="number" label="Stok Minimum" />
          <BaseInput v-model="form.unit" label="Satuan" placeholder="pcs" />
        </div>

        <div class="flex justify-end gap-2 pt-3 border-t border-gray-100 dark:border-gray-800">
          <BaseButton type="button" variant="secondary" @click="showModal = false">Batal</BaseButton>
          <BaseButton type="submit">{{ isEdit ? "Simpan" : "Tambah" }}</BaseButton>
        </div>
      </form>
    </BaseModal>

    <BaseModal v-model="showDeleteModal" title="Hapus Produk">
      <div class="select-none">
        <p class="text-sm text-gray-500 dark:text-gray-400 mb-5">
          Yakin ingin menghapus produk <span class="font-bold text-gray-900 dark:text-white">[{{ deleteTarget?.name }}]</span>? Tindakan ini bersifat permanen.
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
import { ref, reactive, onMounted } from "vue";
import { useProductStore } from "../stores/productStore";
import { useCategoryStore } from "../stores/categoryStore";
import BaseButton from "../components/ui/BaseButton.vue";
import BaseInput from "../components/ui/BaseInput.vue";
import BaseModal from "../components/ui/BaseModal.vue";
import BaseTable from "../components/ui/BaseTable.vue";
import {
  MagnifyingGlassIcon,
  PlusIcon,
  ExclamationTriangleIcon,
  PencilSquareIcon,
  TrashIcon,
  ArrowPathIcon,
} from "@heroicons/vue/24/outline";

const productStore = useProductStore();
const categoryStore = useCategoryStore();

const keyword = ref("");
const showModal = ref(false);
const showDeleteModal = ref(false);
const isEdit = ref(false);
const editId = ref(null);
const deleteTarget = ref(null);

const columns = [
  { key: "name", label: "Nama Produk" },
  { key: "barcode", label: "Barcode" },
  { key: "category_name", label: "Kategori" },
  { key: "buy_price", label: "Harga Beli" },
  { key: "sell_price", label: "Harga Jual" },
  { key: "stock", label: "Stok" },
];

const emptyForm = () => ({
  name: "",
  barcode: "",
  category_id: null,
  buy_price: 0,
  sell_price: 0,
  stock: 0,
  min_stock: 0,
  unit: "pcs",
});

const form = reactive(emptyForm());

function formatRupiah(value) {
  return new Intl.NumberFormat("id-ID", { style: "currency", currency: "IDR", minimumFractionDigits: 0 }).format(value || 0);
}

function generateBarcode() {
  const digits = Math.floor(Math.random() * 100000000).toString().padStart(8, "0");
  form.barcode = "TK" + digits;
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
    barcode: row.barcode || "",
    category_id: row.category_id,
    buy_price: row.buy_price,
    sell_price: row.sell_price,
    stock: row.stock,
    min_stock: row.min_stock,
    unit: row.unit,
  });
  isEdit.value = true;
  editId.value = row.id;
  showModal.value = true;
}

async function submit() {
  if (isEdit.value) {
    await productStore.update(editId.value, form);
  } else {
    await productStore.add(form);
  }
  showModal.value = false;
}

function confirmDelete(row) {
  deleteTarget.value = row;
  showDeleteModal.value = true;
}

async function doDelete() {
  if (deleteTarget.value) {
    await productStore.remove(deleteTarget.value.id);
  }
  showDeleteModal.value = false;
}

function search() {
  productStore.fetchAll(keyword.value);
}

onMounted(async () => {
  await categoryStore.fetchAll();
  await productStore.fetchAll();
});
</script>