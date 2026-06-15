<template>
  <div class="space-y-4">
    <!-- Tab Navigasi -->
    <div class="flex gap-4 border-b border-gray-200 dark:border-gray-800 select-none">
      <button
        v-for="tab in tabs"
        :key="tab.key"
        class="flex items-center gap-2 px-4 py-3 text-sm font-bold border-b-2 transition-colors relative -mb-px"
        :class="activeTab === tab.key
          ? 'border-blue-600 text-blue-600 dark:text-blue-400 dark:border-blue-400'
          : 'border-transparent text-gray-500 hover:text-gray-800 dark:hover:text-gray-200'"
        @click="activeTab = tab.key"
      >
        <component :is="tab.icon" class="w-4 h-4" />
        <span>{{ tab.label }}</span>
      </button>
    </div>

    <!-- Tab: Riwayat Pergerakan -->
    <div v-if="activeTab === 'masuk'" class="space-y-4">
      <div class="flex justify-end select-none">
        <BaseButton class="flex items-center gap-1.5" @click="openStockIn">
          <PlusIcon class="w-4 h-4" />
          <span>Catat Barang Masuk</span>
        </BaseButton>
      </div>

      <BaseTable :columns="movementColumns" :rows="paginatedRows">
        <template #cell-movement_type="{ row }">
          <span
            class="px-2.5 py-1 rounded-md text-xs font-bold border"
            :class="row.movement_type === 'IN'
              ? 'bg-green-50 text-green-600 border-green-200 dark:bg-green-900/20 dark:text-green-400 dark:border-green-900/50'
              : 'bg-red-50 text-red-600 border-red-200 dark:bg-red-900/20 dark:text-red-400 dark:border-red-900/50'"
          >
            {{ row.movement_type === "IN" ? "Masuk" : "Keluar" }}
          </span>
        </template>
        <template #cell-date="{ row }">{{ formatDate(row.date) }}</template>
        <template #cell-supplier_name="{ row }">{{ row.supplier_name || "-" }}</template>
        <template #cell-reference="{ row }">{{ row.reference || "-" }}</template>
        <template #cell-notes="{ row }">{{ row.notes || "-" }}</template>
      </BaseTable>
      <BasePagination
        v-model="currentPage"
        :total="movements.length"
        :per-page="perPage"
      />

      <p v-if="loadingMovements" class="text-xs font-medium text-gray-400 px-1 select-none">Memuat riwayat sistem...</p>

      <div v-if="!loadingMovements && movements.length === 0" class="bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-lg p-12 text-center text-gray-400 select-none">
        <ArchiveBoxIcon class="w-10 h-10 mx-auto mb-2 opacity-30" />
        <p class="text-sm font-bold text-gray-500 dark:text-gray-400">Belum ada riwayat pergerakan stok</p>
      </div>
    </div>

    <!-- Tab: Stok Menipis -->
    <div v-if="activeTab === 'menipis'" class="space-y-4">
      <div v-if="lowStockItems.length === 0" class="bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-lg p-12 text-center text-gray-400 select-none">
        <CheckCircleIcon class="w-10 h-10 mx-auto mb-2 text-green-500 opacity-60" />
        <p class="text-sm font-bold text-gray-900 dark:text-white">Semua stok aman!</p>
        <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">Tidak ada produk yang stoknya di bawah batas minimum</p>
      </div>
      <div v-else class="space-y-3">
        <div
          v-for="item in lowStockItems"
          :key="item.id"
          class="bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-lg p-4 flex items-center justify-between border-l-4 select-none"
          :class="item.stock === 0 ? 'border-l-red-500' : 'border-l-amber-500'"
        >
          <div>
            <p class="font-bold text-gray-900 dark:text-white">{{ item.name }}</p>
            <p class="text-xs font-semibold text-gray-400 dark:text-gray-500 mt-0.5">Minimum: {{ item.min_stock }} {{ item.unit }}</p>
          </div>
          <div class="text-right flex flex-col items-end">
            <span
              class="text-sm font-bold px-2.5 py-1 rounded-md border"
              :class="item.stock === 0
                ? 'bg-red-50 border-red-200 text-red-600 dark:bg-red-900/20 dark:border-red-900/40 dark:text-red-400'
                : 'bg-amber-50 border-amber-200 text-amber-600 dark:bg-amber-900/20 dark:border-amber-900/40 dark:text-amber-400'"
            >
              Sisa {{ item.stock }} {{ item.unit }}
            </span>
            <span v-if="item.stock === 0" class="text-[10px] font-black text-red-500 tracking-wider mt-1 uppercase">Habis</span>
            <span v-else class="text-[10px] font-black text-amber-500 tracking-wider mt-1 uppercase">Menipis</span>
          </div>
        </div>
      </div>
    </div>

    <!-- Modal Catat Barang Masuk -->
    <BaseModal v-model="showStockInModal" title="Catat Barang Masuk">
      <form class="space-y-4" @submit.prevent="submitStockIn">

        <!-- Pencarian Produk: ketik nama atau scan barcode -->
        <div>
          <label class="block text-xs font-bold text-gray-700 dark:text-gray-300 uppercase tracking-wider mb-1.5">
            Produk <span class="text-red-500">*</span>
          </label>

          <!-- Field search / scan -->
          <div class="relative">
            <input
              v-model="productSearch"
              type="text"
              data-barcode-input="true"
              class="w-full bg-white dark:bg-gray-900 border rounded-md pl-9 pr-4 py-2 text-sm font-medium text-gray-900 dark:text-white focus:outline-none transition-colors"
              :class="selectedProduct
                ? 'border-green-400 dark:border-green-600'
                : 'border-gray-200 dark:border-gray-800 focus:border-blue-500 dark:focus:border-blue-400'"
              placeholder="Ketik nama produk atau scan barcode..."
              @input="onProductSearchInput"
              @keydown.enter.prevent="pickFirstSuggestion"
              @focus="showSuggestions = true"
              @blur="onSearchBlur"
            />
            <MagnifyingGlassIcon class="w-4 h-4 absolute left-3 top-2.5 text-gray-400 pointer-events-none" />

            <!-- Dropdown hasil pencarian -->
            <div
              v-if="showSuggestions && suggestions.length > 0"
              class="absolute z-20 w-full mt-1 bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-700 rounded-md shadow-lg max-h-48 overflow-y-auto"
            >
              <button
                v-for="p in suggestions"
                :key="p.id"
                type="button"
                class="w-full text-left px-4 py-2.5 text-sm hover:bg-blue-50 dark:hover:bg-blue-900/20 transition-colors flex items-center justify-between gap-2"
                @mousedown.prevent="selectProduct(p)"
              >
                <span class="font-medium text-gray-900 dark:text-white">{{ p.name }}</span>
                <span class="text-xs text-gray-400 whitespace-nowrap">Stok: {{ p.stock }} {{ p.unit }}</span>
              </button>
            </div>
          </div>

          <!-- Produk terpilih -->
          <div v-if="selectedProduct" class="mt-2 flex items-center justify-between bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800/50 rounded-md px-3 py-2">
            <div>
              <p class="text-sm font-bold text-blue-800 dark:text-blue-300">{{ selectedProduct.name }}</p>
              <p class="text-xs text-blue-600 dark:text-blue-400">Stok saat ini: {{ selectedProduct.stock }} {{ selectedProduct.unit }}</p>
            </div>
            <button type="button" class="text-blue-400 hover:text-blue-600 dark:hover:text-blue-200" @click="clearSelectedProduct">
              <XMarkIcon class="w-4 h-4" />
            </button>
          </div>

          <p v-if="!selectedProduct" class="text-xs text-gray-400 mt-1">Bisa ketik nama atau scan barcode produk langsung ke field ini.</p>
        </div>

        <BaseInput
          v-model="stockInForm.quantity"
          type="number"
          min="1"
          label="Jumlah Masuk"
          required
          placeholder="10"
        />

        <!-- Supplier -->
        <div>
          <label class="block text-xs font-bold text-gray-700 dark:text-gray-300 uppercase tracking-wider mb-1.5">Supplier</label>
          <select
            v-model="stockInForm.supplier_id"
            class="w-full bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-md px-3 py-2 text-sm font-medium text-gray-900 dark:text-white focus:outline-none focus:border-blue-500 dark:focus:border-blue-400 transition-colors"
          >
            <option :value="null">- Pilih supplier (opsional) -</option>
            <option v-for="s in suppliers" :key="s.id" :value="s.id">{{ s.name }}</option>
          </select>
        </div>

        <BaseInput
          v-model="stockInForm.reference"
          label="No. Referensi / Faktur"
          placeholder="FAK-001 (opsional)"
        />

        <div>
          <label class="block text-xs font-bold text-gray-700 dark:text-gray-300 uppercase tracking-wider mb-1.5">Catatan</label>
          <textarea
            v-model="stockInForm.notes"
            class="w-full bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-md px-3 py-2 text-sm font-medium text-gray-900 dark:text-white focus:outline-none focus:border-blue-500 dark:focus:border-blue-400 transition-colors resize-none"
            rows="2"
            placeholder="Catatan tambahan..."
          ></textarea>
        </div>

        <div v-if="stockInError" class="flex items-center gap-2 text-xs font-bold text-red-600 bg-red-50 border border-red-100 dark:bg-red-900/20 dark:border-red-900/40 dark:text-red-400 p-3 rounded-md">
          <ExclamationTriangleIcon class="w-4 h-4 flex-shrink-0" />
          <span>{{ stockInError }}</span>
        </div>

        <div class="flex justify-end gap-2 pt-3 border-t border-gray-100 dark:border-gray-800">
          <BaseButton type="button" variant="secondary" @click="showStockInModal = false">Batal</BaseButton>
          <BaseButton type="submit" :disabled="submitting">
            {{ submitting ? "Menyimpan..." : "Simpan" }}
          </BaseButton>
        </div>
      </form>
    </BaseModal>
  </div>
</template>

<script setup>
import { ref, reactive, computed, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useProductStore } from "../stores/productStore";
import { useSupplierStore } from "../stores/supplierStore";
import { useBarcode } from "../composables/useBarcode";
import BaseButton from "../components/ui/BaseButton.vue";
import BaseInput from "../components/ui/BaseInput.vue";
import BaseModal from "../components/ui/BaseModal.vue";
import BaseTable from "../components/ui/BaseTable.vue";
import BasePagination from "../components/ui/BasePagination.vue";
import {
  ClipboardDocumentListIcon,
  ExclamationTriangleIcon,
  PlusIcon,
  ArchiveBoxIcon,
  CheckCircleIcon,
  MagnifyingGlassIcon,
  XMarkIcon,
} from "@heroicons/vue/24/outline";

const productStore = useProductStore();
const supplierStore = useSupplierStore();

const activeTab = ref("masuk");
const tabs = [
  { key: "masuk", label: "Riwayat Pergerakan", icon: ClipboardDocumentListIcon },
  { key: "menipis", label: "Stok Menipis", icon: ExclamationTriangleIcon },
];

const movements = ref([]);
const loadingMovements = ref(false);
const lowStockItems = ref([]);

const showStockInModal = ref(false);
const submitting = ref(false);
const stockInError = ref("");

// State pencarian produk
const productSearch = ref("");
const selectedProduct = ref(null);
const showSuggestions = ref(false);

const movementColumns = [
  { key: "product_name", label: "Produk" },
  { key: "movement_type", label: "Tipe" },
  { key: "quantity", label: "Jumlah" },
  { key: "supplier_name", label: "Supplier" },
  { key: "reference", label: "Referensi" },
  { key: "date", label: "Tanggal" },
  { key: "notes", label: "Catatan" },
];

const emptyStockInForm = () => ({
  product_id: null,
  quantity: 1,
  supplier_id: null,
  reference: "",
  notes: "",
});

const stockInForm = reactive(emptyStockInForm());
const suppliers = ref([]);

// Filter produk berdasarkan keyword ketikan
const suggestions = computed(() => {
  const q = productSearch.value.trim().toLowerCase();
  if (!q || selectedProduct.value) return [];
  return productStore.products
    .filter((p) => p.name.toLowerCase().includes(q))
    .slice(0, 8);
});

const currentPage = ref(1);
const perPage = 20;

// Sesuaikan nama array datanya (movements / suppliers / orders / services)
const paginatedRows = computed(() => {
  const start = (currentPage.value - 1) * perPage;
  return movements.value.slice(start, start + perPage);
});

function onProductSearchInput() {
  selectedProduct.value = null;
  stockInForm.product_id = null;
  showSuggestions.value = true;
}

function selectProduct(p) {
  selectedProduct.value = p;
  stockInForm.product_id = p.id;
  productSearch.value = p.name;
  showSuggestions.value = false;
}

function clearSelectedProduct() {
  selectedProduct.value = null;
  stockInForm.product_id = null;
  productSearch.value = "";
  showSuggestions.value = false;
}

function pickFirstSuggestion() {
  if (suggestions.value.length > 0) {
    selectProduct(suggestions.value[0]);
  }
}

// Delay blur supaya klik dropdown sempat terpicu dulu
function onSearchBlur() {
  setTimeout(() => {
    showSuggestions.value = false;
  }, 150);
}

function formatDate(dateStr) {
  if (!dateStr) return "-";
  const d = new Date(dateStr.replace(" ", "T") + "Z");
  return d.toLocaleString("id-ID", {
    day: "numeric",
    month: "short",
    year: "numeric",
    hour: "2-digit",
    minute: "2-digit",
  });
}

async function fetchMovements() {
  loadingMovements.value = true;
  try {
    movements.value = await invoke("get_stock_movements", { productId: null, limit: 200 });
  } catch (err) {
    console.error(err);
  } finally {
    loadingMovements.value = false;
  }
}

async function fetchLowStock() {
  try {
    lowStockItems.value = await invoke("get_low_stock_products");
  } catch (err) {
    console.error(err);
  }
}

function openStockIn() {
  Object.assign(stockInForm, emptyStockInForm());
  productSearch.value = "";
  selectedProduct.value = null;
  stockInError.value = "";
  showStockInModal.value = true;
}

async function submitStockIn() {
  if (!stockInForm.product_id) {
    stockInError.value = "Pilih produk terlebih dahulu";
    return;
  }
  if (Number(stockInForm.quantity) <= 0) {
    stockInError.value = "Jumlah harus lebih dari 0";
    return;
  }

  submitting.value = true;
  stockInError.value = "";
  try {
    await invoke("add_stock_in", {
      payload: {
        product_id: stockInForm.product_id,
        quantity: Number(stockInForm.quantity),
        supplier_id: stockInForm.supplier_id,
        reference: stockInForm.reference || null,
        notes: stockInForm.notes || null,
      },
    });
    showStockInModal.value = false;
    await Promise.all([fetchMovements(), fetchLowStock(), productStore.fetchAll()]);
  } catch (err) {
    stockInError.value = err?.toString() ?? "Gagal menyimpan";
  } finally {
    submitting.value = false;
  }
}

// Scan barcode langsung pilih produk di modal
useBarcode(async (barcode) => {
  if (!showStockInModal.value) return;
  const product = await productStore.findByBarcode(barcode);
  if (product) {
    selectProduct(product);
  } else {
    stockInError.value = `Produk dengan barcode "${barcode}" tidak ditemukan`;
  }
});

onMounted(async () => {
  await Promise.all([
    productStore.fetchAll(),
    supplierStore.fetchAll(),
    fetchMovements(),
    fetchLowStock(),
  ]);
  suppliers.value = supplierStore.suppliers;
});
</script>