<template>
  <div class="flex flex-col lg:flex-row gap-4 h-full min-h-0">
    <div class="flex-1 flex flex-col gap-4 min-h-0">
      <div class="relative flex-shrink-0 select-none">
        <input
          ref="searchInput"
          v-model="keyword"
          type="text"
          class="w-full bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-md pl-10 pr-4 py-2.5 text-sm font-medium text-gray-900 dark:text-white placeholder-gray-400 focus:outline-none focus:border-blue-500 dark:focus:border-blue-400 transition-colors"
          placeholder="Scan barcode atau cari nama produk..."
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
          <span>Tambah Percetakan / Jasa</span>
        </BaseButton>
      </div>

      <div class="flex-1 overflow-y-auto pr-1">
        <ProductGrid :products="productStore.products" @add="addToCart" />
      </div>
    </div>

    <div class="w-full lg:w-96 flex-shrink-0 bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-lg p-4 flex flex-col gap-4 select-none">
      <div class="flex items-center gap-2 border-b border-gray-100 dark:border-gray-800 pb-3">
        <ShoppingCartIcon class="w-5 h-5 text-gray-500" />
        <h3 class="font-bold text-gray-900 dark:text-white">Keranjang</h3>
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

    <BaseModal v-model="showSuccessModal" title="Transaksi Berhasil">
      <div class="text-center py-6 select-none">
        <div class="inline-flex items-center justify-center p-3 bg-green-50 dark:bg-green-900/20 border border-green-100 dark:border-green-900/40 rounded-md mb-4">
          <CheckCircleIcon class="w-12 h-12 text-green-600 dark:text-green-400" />
        </div>
        <p class="text-xs font-bold text-gray-400 tracking-wider uppercase mb-0.5">No. Invoice</p>
        <p class="text-xl font-black text-gray-900 dark:text-white mb-6">{{ posStore.lastResult?.invoice_no }}</p>
        <BaseButton class="w-full" @click="showSuccessModal = false">Transaksi Baru</BaseButton>
      </div>
    </BaseModal>
  </div>
</template>

<script setup>
import { ref, reactive, onMounted, watch } from "vue";
import { useProductStore } from "../stores/productStore";
import { usePosStore } from "../stores/posStore";
import { useBarcode } from "../composables/useBarcode";
import ProductGrid from "../components/pos/ProductGrid.vue";
import CartItem from "../components/pos/CartItem.vue";
import PaymentPanel from "../components/pos/PaymentPanel.vue";
import BaseButton from "../components/ui/BaseButton.vue";
import BaseInput from "../components/ui/BaseInput.vue";
import BaseModal from "../components/ui/BaseModal.vue";
import { 
  MagnifyingGlassIcon, 
  PlusIcon, 
  ShoppingCartIcon, 
  CheckCircleIcon 
} from "@heroicons/vue/24/outline";

const productStore = useProductStore();
const posStore = usePosStore();

const keyword = ref("");
const searchInput = ref(null);
const showServiceModal = ref(false);
const showSuccessModal = ref(false);

const serviceForm = reactive({
  item_name: "",
  quantity: 1,
  unit_price: 0,
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

async function doCheckout() {
  const success = await posStore.checkout();
  if (success) {
    showSuccessModal.value = true;
    await productStore.fetchAll(keyword.value);
  }
}

watch(showSuccessModal, (val) => {
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