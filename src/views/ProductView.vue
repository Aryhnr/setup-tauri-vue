<template>
  <div class="space-y-4">
    <!-- Toolbar -->
    <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-3">
      <div class="relative w-full sm:w-80">
        <input
          v-model="keyword"
          type="text"
          class="input pl-9"
          placeholder="Cari nama produk atau scan barcode..."
          @keyup.enter="search"
        />
        <span class="absolute left-3 top-2.5 text-gray-400">🔍</span>
      </div>
      <BaseButton @click="openCreate">+ Tambah Produk</BaseButton>
    </div>

    <!-- Alert stok menipis -->
    <div
      v-if="productStore.lowStockProducts.length"
      class="card p-4 border-yellow-300 bg-yellow-50 dark:bg-yellow-900/20 dark:border-yellow-700"
    >
      <p class="text-sm font-medium text-yellow-700 dark:text-yellow-300">
        ⚠️ {{ productStore.lowStockProducts.length }} produk stoknya menipis:
        {{ productStore.lowStockProducts.map((p) => p.name).join(", ") }}
      </p>
    </div>

    <!-- Tabel produk -->
    <BaseTable :columns="columns" :rows="productStore.products">
      <template #cell-stock="{ row }">
        <span
          class="px-2 py-0.5 rounded-full text-xs font-semibold"
          :class="row.stock <= row.min_stock
            ? 'bg-red-100 text-red-600 dark:bg-red-900/40 dark:text-red-300'
            : 'bg-green-100 text-green-600 dark:bg-green-900/40 dark:text-green-300'"
        >
          {{ row.stock }} {{ row.unit }}
        </span>
      </template>

      <template #cell-buy_price="{ row }">{{ formatRupiah(row.buy_price) }}</template>
      <template #cell-sell_price="{ row }">{{ formatRupiah(row.sell_price) }}</template>
      <template #cell-category_name="{ row }">{{ row.category_name || "-" }}</template>

      <template #actions="{ row }">
        <div class="flex justify-end gap-2">
          <button class="text-blue-600 hover:underline text-sm" @click="openEdit(row)">Edit</button>
          <button class="text-red-500 hover:underline text-sm" @click="confirmDelete(row)">Hapus</button>
        </div>
      </template>
    </BaseTable>

    <p v-if="productStore.loading" class="text-sm text-gray-400">Memuat data...</p>
    <p v-if="productStore.error" class="text-sm text-red-500">{{ productStore.error }}</p>

    <!-- Modal Tambah / Edit -->
    <BaseModal v-model="showModal" :title="isEdit ? 'Edit Produk' : 'Tambah Produk'">
      <form class="space-y-3" @submit.prevent="submit">
        <BaseInput v-model="form.name" label="Nama Produk" required placeholder="Indomie Goreng" />
        <BaseInput v-model="form.barcode" label="Barcode" placeholder="(opsional) scan barcode" />

        <div>
          <label class="label">Kategori</label>
          <select v-model="form.category_id" class="input">
            <option :value="null">- Pilih kategori -</option>
            <option v-for="c in categoryStore.categories" :key="c.id" :value="c.id">{{ c.name }}</option>
          </select>
        </div>

        <div class="grid grid-cols-2 gap-3">
          <BaseInput v-model="form.buy_price" type="number" step="100" label="Harga Beli" />
          <BaseInput v-model="form.sell_price" type="number" step="100" label="Harga Jual" />
        </div>

        <div class="grid grid-cols-3 gap-3">
          <BaseInput v-model="form.stock" type="number" label="Stok" />
          <BaseInput v-model="form.min_stock" type="number" label="Stok Minimum" />
          <BaseInput v-model="form.unit" label="Satuan" placeholder="pcs" />
        </div>

        <div class="flex justify-end gap-2 pt-2">
          <BaseButton type="button" variant="secondary" @click="showModal = false">Batal</BaseButton>
          <BaseButton type="submit">{{ isEdit ? "Simpan" : "Tambah" }}</BaseButton>
        </div>
      </form>
    </BaseModal>

    <!-- Modal konfirmasi hapus -->
    <BaseModal v-model="showDeleteModal" title="Hapus Produk">
      <p class="text-sm text-gray-500 mb-4">
        Yakin ingin menghapus produk <strong>{{ deleteTarget?.name }}</strong>? Tindakan ini tidak dapat dibatalkan.
      </p>
      <div class="flex justify-end gap-2">
        <BaseButton variant="secondary" @click="showDeleteModal = false">Batal</BaseButton>
        <BaseButton variant="danger" @click="doDelete">Hapus</BaseButton>
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
  return new Intl.NumberFormat("id-ID", { style: "currency", currency: "IDR", minimumFractionDigits: 0 }).format(
    value || 0
  );
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