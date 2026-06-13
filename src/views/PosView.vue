<template>
  <div class="flex flex-col lg:flex-row gap-4 h-full">
    <!-- Kiri: pencarian + grid produk -->
    <div class="flex-1 flex flex-col gap-3 min-h-0">
      <div class="relative">
        <input
          ref="searchInput"
          v-model="keyword"
          type="text"
          class="input pl-9"
          placeholder="Scan barcode atau cari nama produk..."
          data-barcode-input="true"
          @keyup.enter="search"
        />
        <span class="absolute left-3 top-2.5 text-gray-400">🔍</span>
      </div>

      <div class="flex gap-2">
        <BaseButton variant="secondary" class="text-xs" @click="showServiceModal = true">
          + Tambah Percetakan / Jasa
        </BaseButton>
      </div>

      <div class="flex-1 overflow-y-auto pr-1">
        <ProductGrid :products="productStore.products" @add="addToCart" />
      </div>
    </div>

    <!-- Kanan: keranjang + pembayaran -->
    <div class="w-full lg:w-96 flex-shrink-0 card p-4 flex flex-col gap-3">
      <h3 class="font-semibold flex items-center gap-2">🛒 Keranjang</h3>

      <div class="flex-1 overflow-y-auto min-h-[120px] max-h-[40vh]">
        <CartItem
          v-for="(item, index) in posStore.cart"
          :key="`${item.product_id ?? 'custom'}-${index}`"
          :item="item"
          @update-quantity="(qty) => posStore.updateQuantity(index, qty)"
          @remove="posStore.removeItem(index)"
        />
        <p v-if="posStore.cart.length === 0" class="text-center text-gray-400 text-sm py-8">
          Keranjang masih kosong
        </p>
      </div>

      <hr class="border-gray-100 dark:border-gray-700" />

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

    <!-- Modal tambah Percetakan / Jasa -->
    <BaseModal v-model="showServiceModal" title="Tambah Percetakan / Jasa">
      <form class="space-y-3" @submit.prevent="addServiceItem">
        <BaseInput v-model="serviceForm.item_name" label="Nama Layanan" required placeholder="Print A4 Hitam Putih" />
        <div class="grid grid-cols-2 gap-3">
          <BaseInput v-model="serviceForm.quantity" type="number" label="Jumlah" />
          <BaseInput v-model="serviceForm.unit_price" type="number" step="100" label="Harga Satuan" />
        </div>
        <div class="flex justify-end gap-2 pt-2">
          <BaseButton type="button" variant="secondary" @click="showServiceModal = false">Batal</BaseButton>
          <BaseButton type="submit">Tambah ke Keranjang</BaseButton>
        </div>
      </form>
    </BaseModal>

    <!-- Modal sukses transaksi -->
    <BaseModal v-model="showSuccessModal" title="Transaksi Berhasil">
      <div class="text-center py-4">
        <p class="text-4xl mb-2">✅</p>
        <p class="text-sm text-gray-500">No. Invoice</p>
        <p class="text-lg font-bold mb-4">{{ posStore.lastResult?.invoice_no }}</p>
        <BaseButton @click="showSuccessModal = false">Transaksi Baru</BaseButton>
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

// Reset keranjang saat modal sukses ditutup, biar siap transaksi baru
watch(showSuccessModal, (val) => {
  if (!val) {
    posStore.lastResult = null;
    keyword.value = "";
    productStore.fetchAll();
    searchInput.value?.focus();
  }
});

// Integrasi barcode scanner USB: hasil scan langsung dicari & ditambahkan
// ke keranjang jika produk ditemukan.
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