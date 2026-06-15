<template>
  <div class="flex flex-col lg:flex-row gap-4 h-full min-h-0 relative">
    <!-- Kiri: Produk -->
    <div class="flex-1 flex flex-col gap-4 min-h-0">
      <div class="relative flex-shrink-0 select-none">
        <input
          ref="searchInput"
          v-model="keyword"
          type="text"
          class="w-full bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-md pl-10 pr-4 py-2.5 text-sm font-medium text-gray-900 dark:text-white placeholder-gray-400 focus:outline-none focus:border-blue-500 dark:focus:border-blue-400 transition-colors"
          placeholder="Scan barcode atau cari nama produk... (F1)"
          data-barcode-input="true"
          @keyup.enter="search"
        />
        <div class="absolute left-3 top-3 flex items-center pointer-events-none">
          <MagnifyingGlassIcon class="w-4 h-4 text-gray-400" />
        </div>
      </div>

      <div class="flex gap-2 flex-shrink-0 select-none">
        <BaseButton variant="secondary" class="text-xs flex items-center gap-1.5" @click="showServiceModal = true">
          <PlusIcon class="w-3.5 h-3.5" />
          <span>Tambah Percetakan / Jasa <span class="opacity-50 ml-1">F3</span></span>
        </BaseButton>
      </div>

      <div class="flex-1 overflow-y-auto pr-1">
        <ProductGrid :products="productStore.products" @add="addToCart" />
      </div>
    </div>

    <!-- Kanan: Keranjang -->
    <div class="w-full lg:w-96 flex-shrink-0 bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-lg p-4 flex flex-col gap-4 select-none">
      <div class="flex items-center justify-between border-b border-gray-100 dark:border-gray-800 pb-3">
        <div class="flex items-center gap-2">
          <ShoppingCartIcon class="w-5 h-5 text-gray-500" />
          <h3 class="font-bold text-gray-900 dark:text-white">Keranjang</h3>
        </div>
        <button
          v-if="posStore.cart.length > 0"
          class="text-xs text-red-400 hover:text-red-600 font-semibold flex items-center gap-1 px-2 py-1 rounded hover:bg-red-50 dark:hover:bg-red-900/20 transition-colors"
          @click="confirmClearCart"
        >
          <TrashIcon class="w-3.5 h-3.5" />
          Kosongkan <span class="opacity-50">F12</span>
        </button>
      </div>

      <div class="flex-1 overflow-y-auto min-h-[120px] max-h-[40vh] lg:max-h-none space-y-1">
        <CartItem
          v-for="(item, index) in posStore.cart"
          :key="`${item.product_id ?? 'custom'}-${index}`"
          :item="item"
          @update-quantity="(qty) => posStore.updateQuantity(index, qty)"
          @remove="posStore.removeItem(index)"
        />
        <div v-if="posStore.cart.length === 0" class="h-full flex flex-col items-center justify-center text-gray-400 py-12">
          <ShoppingCartIcon class="w-8 h-8 mb-2 opacity-30" />
          <p class="text-sm font-medium">Keranjang masih kosong</p>
        </div>
      </div>

      <hr class="border-gray-100 dark:border-gray-800" />

      <PaymentPanel
        ref="paymentPanelRef"
        :total="posStore.total"
        :paid-amount="posStore.paidAmount"
        :change-amount="posStore.changeAmount"
        :can-checkout="posStore.isPaidEnough && posStore.cart.length > 0"
        :processing="posStore.processing"
        :error="posStore.error"
        @update:paid-amount="(val) => (posStore.paidAmount = val)"
        @checkout="doCheckout"
        @clear="posStore.clearCart"
      />
    </div>

    <!-- Cheat sheet shortcut (toggle) -->
    <div class="fixed bottom-6 left-6 z-40 select-none">
      <Transition name="slide-up">
        <div
          v-if="showShortcutSheet"
          class="mb-2 bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-700 rounded-xl shadow-xl p-4 w-64"
        >
          <p class="text-xs font-black text-gray-500 dark:text-gray-400 uppercase tracking-wider mb-3">Shortcut Keyboard</p>
          <div class="space-y-2">
            <div v-for="sc in shortcuts" :key="sc.key" class="flex items-center justify-between">
              <span class="text-xs text-gray-600 dark:text-gray-300">{{ sc.desc }}</span>
              <kbd class="text-xs bg-gray-100 dark:bg-gray-800 border border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300 px-2 py-0.5 rounded font-mono">{{ sc.key }}</kbd>
            </div>
          </div>
        </div>
      </Transition>
      <button
        class="flex items-center gap-2 px-3 py-2 text-xs font-semibold bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-700 text-gray-600 dark:text-gray-300 rounded-lg shadow hover:shadow-md transition-all"
        @click="showShortcutSheet = !showShortcutSheet"
      >
        <CommandLineIcon class="w-4 h-4" />
        Shortcut
      </button>
    </div>

    <!-- Modal Tambah Percetakan / Jasa -->
    <BaseModal v-model="showServiceModal" title="Tambah Percetakan / Jasa">
      <form class="space-y-4" @submit.prevent="addServiceItem">
        <BaseInput v-model="serviceForm.item_name" label="Nama Layanan" required placeholder="Print A4 Hitam Putih" />
        <div class="grid grid-cols-2 gap-4">
          <BaseInput v-model="serviceForm.quantity" type="number" label="Jumlah" />
          <BaseInput v-model="serviceForm.unit_price" type="number" step="100" label="Harga Satuan" />
        </div>
        <div class="flex justify-end gap-2 pt-2 border-t border-gray-100 dark:border-gray-800">
          <BaseButton type="button" variant="secondary" @click="showServiceModal = false">Batal</BaseButton>
          <BaseButton type="submit">Tambah ke Keranjang</BaseButton>
        </div>
      </form>
    </BaseModal>

    <!-- Modal Konfirmasi Kosongkan Keranjang -->
    <BaseModal v-model="showClearModal" title="Kosongkan Keranjang?">
      <p class="text-sm text-gray-500 dark:text-gray-400 mb-4">
        Semua item di keranjang akan dihapus. Lanjutkan?
      </p>
      <div class="flex gap-2">
        <BaseButton variant="secondary" class="flex-1" @click="showClearModal = false">Batal</BaseButton>
        <BaseButton variant="danger" class="flex-1" @click="doClearCart">Kosongkan</BaseButton>
      </div>
    </BaseModal>

    <!-- Modal Struk -->
    <ReceiptModal
      v-model="showReceiptModal"
      :transaction="receiptTransaction"
      :items="receiptItems"
    />
  </div>
</template>

<script setup>
import { ref, reactive, onMounted, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useProductStore } from "../stores/productStore";
import { usePosStore } from "../stores/posStore";
import { useBarcode } from "../composables/useBarcode";
import { useKeyboardShortcut } from "../composables/useKeyboardShortcut";
import ProductGrid from "../components/pos/ProductGrid.vue";
import CartItem from "../components/pos/CartItem.vue";
import PaymentPanel from "../components/pos/PaymentPanel.vue";
import ReceiptModal from "../components/pos/ReceiptModal.vue";
import BaseButton from "../components/ui/BaseButton.vue";
import BaseInput from "../components/ui/BaseInput.vue";
import BaseModal from "../components/ui/BaseModal.vue";
import {
  MagnifyingGlassIcon,
  PlusIcon,
  ShoppingCartIcon,
  TrashIcon,
  CommandLineIcon,
} from "@heroicons/vue/24/outline";

const productStore = useProductStore();
const posStore = usePosStore();

const keyword = ref("");
const searchInput = ref(null);
const paymentPanelRef = ref(null);
const showServiceModal = ref(false);
const showClearModal = ref(false);
const showShortcutSheet = ref(false);
const showReceiptModal = ref(false);

const receiptTransaction = ref(null);
const receiptItems = ref([]);

const serviceForm = reactive({
  item_name: "",
  quantity: 1,
  unit_price: 0,
});

const shortcuts = [
  { key: "F1", desc: "Fokus pencarian produk" },
  { key: "F2", desc: "Fokus field jumlah bayar" },
  { key: "F3", desc: "Tambah percetakan/jasa" },
  { key: "F12", desc: "Kosongkan keranjang" },
  { key: "Enter", desc: "Proses bayar (di field bayar)" },
  { key: "Esc", desc: "Tutup modal / panel" },
];

// Pasang shortcut keyboard
useKeyboardShortcut({
  onF1() {
    showShortcutSheet.value = false;
    searchInput.value?.focus();
    searchInput.value?.select();
  },
  onF2() {
    showShortcutSheet.value = false;
    // Fokus ke input bayar di PaymentPanel
    const payInput = document.querySelector("input[data-shortcut-target='bayar']");
    payInput?.focus();
    payInput?.select();
  },
  onF3() {
    showClearModal.value = false;
    showServiceModal.value = true;
  },
  onF12() {
    if (posStore.cart.length > 0) {
      confirmClearCart();
    }
  },
  onEscape() {
    if (showShortcutSheet.value) {
      showShortcutSheet.value = false;
    } else if (showClearModal.value) {
      showClearModal.value = false;
    } else if (showServiceModal.value) {
      showServiceModal.value = false;
    }
  },
  onEnterBayar() {
    if (posStore.isPaidEnough && posStore.cart.length > 0) {
      // Trigger konfirmasi bayar via PaymentPanel
      const payBtn = document.querySelector("[data-pay-btn]");
      payBtn?.click();
    }
  },
});

function addToCart(product) {
  if (product.stock <= 0) return;
  posStore.addProduct(product, 1);
}

function search() {
  productStore.fetchAll(keyword.value);
}

function addServiceItem() {
  posStore.addCustomItem({
    item_name: serviceForm.item_name,
    quantity: serviceForm.quantity,
    unit_price: serviceForm.unit_price,
  });
  serviceForm.item_name = "";
  serviceForm.quantity = 1;
  serviceForm.unit_price = 0;
  showServiceModal.value = false;
}

function confirmClearCart() {
  showClearModal.value = true;
}

function doClearCart() {
  posStore.clearCart();
  showClearModal.value = false;
}

async function doCheckout() {
  const ok = await posStore.checkout();
  if (ok && posStore.lastResult) {
    try {
      const items = await invoke("get_transaction_detail", {
        transactionId: posStore.lastResult.id,
      });
      receiptItems.value = items;
      receiptTransaction.value = {
        invoice_no: posStore.lastResult.invoice_no,
        transaction_date: new Date().toISOString().replace("T", " ").slice(0, 19),
        total_amount: posStore.lastResult.total_amount ?? 0,
        paid_amount: posStore.lastResult.paid_amount ?? 0,
        change_amount: posStore.lastResult.change_amount ?? 0,
      };
    } catch {
      receiptItems.value = [];
      receiptTransaction.value = {
        invoice_no: posStore.lastResult.invoice_no,
        transaction_date: new Date().toISOString().replace("T", " ").slice(0, 19),
        total_amount: 0, paid_amount: 0, change_amount: 0,
      };
    }
    showReceiptModal.value = true;
    await productStore.fetchAll(keyword.value);
  }
}

watch(showReceiptModal, (val) => {
  if (!val) {
    posStore.lastResult = null;
    keyword.value = "";
    productStore.fetchAll();
    searchInput.value?.focus();
  }
});

useBarcode(async (barcode) => {
  const product = await productStore.findByBarcode(barcode);
  if (product) {
    addToCart(product);
  } else {
    posStore.error = `Produk dengan barcode "${barcode}" tidak ditemukan`;
  }
});

onMounted(async () => {
  await productStore.fetchAll();
  searchInput.value?.focus();
});
</script>

<style scoped>
.slide-up-enter-active,
.slide-up-leave-active {
  transition: all 0.2s ease;
}
.slide-up-enter-from,
.slide-up-leave-to {
  opacity: 0;
  transform: translateY(8px);
}
</style>