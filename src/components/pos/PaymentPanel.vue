<template>
  <div class="space-y-3">
    <div class="flex items-center justify-between text-lg font-bold">
      <span>Total</span>
      <span>{{ formatRupiah(total) }}</span>
    </div>

    <div>
      <label class="label">Jumlah Bayar <span class="text-xs text-gray-400 font-normal ml-1">(F2)</span></label>
      <input
        type="number"
        class="input text-lg font-semibold"
        :value="paidAmount"
        placeholder="0"
        data-shortcut-target="bayar"
        @input="$emit('update:paidAmount', $event.target.value)"
        @keydown.enter.prevent="requestCheckout"
      />
    </div>

    <div class="grid grid-cols-4 gap-2">
      <button
        v-for="amount in quickAmounts"
        :key="amount"
        class="btn-secondary text-xs py-1.5"
        @click="$emit('update:paidAmount', amount)"
      >
        {{ formatRupiahShort(amount) }}
      </button>
    </div>

    <div class="flex items-center justify-between text-sm">
      <span class="text-gray-500">Kembalian</span>
      <span class="font-semibold" :class="changeAmount > 0 ? 'text-green-600 dark:text-green-400' : ''">
        {{ formatRupiah(changeAmount) }}
      </span>
    </div>

    <p v-if="error" class="text-sm text-red-500">{{ error }}</p>

    <div class="grid grid-cols-2 gap-2 pt-1">
      <BaseButton variant="secondary" :disabled="processing" @click="$emit('clear')">Batal</BaseButton>
      <BaseButton data-pay-btn :disabled="!canCheckout || processing" @click="requestCheckout">
        {{ processing ? "Memproses..." : "Bayar" }}
      </BaseButton>
    </div>

    <!-- Modal Konfirmasi Pembayaran -->
    <BaseModal v-model="showConfirm" title="Konfirmasi Pembayaran">
      <div class="space-y-4">
        <div class="bg-gray-50 dark:bg-gray-800 rounded-lg p-4 space-y-2">
          <div class="flex justify-between text-sm">
            <span class="text-gray-500">Total Belanja</span>
            <span class="font-bold text-gray-900 dark:text-white">{{ formatRupiah(total) }}</span>
          </div>
          <div class="flex justify-between text-sm">
            <span class="text-gray-500">Uang Dibayar</span>
            <span class="font-bold text-gray-900 dark:text-white">{{ formatRupiah(Number(paidAmount)) }}</span>
          </div>
          <div class="border-t border-gray-200 dark:border-gray-700 pt-2 flex justify-between text-sm">
            <span class="text-gray-500">Kembalian</span>
            <span class="font-bold text-green-600 dark:text-green-400">{{ formatRupiah(changeAmount) }}</span>
          </div>
        </div>
        <p class="text-sm text-gray-500 dark:text-gray-400 text-center">
          Pastikan jumlah sudah benar sebelum memproses pembayaran.
        </p>
        <div class="grid grid-cols-2 gap-2 pt-1">
          <BaseButton variant="secondary" @click="showConfirm = false">Cek Lagi</BaseButton>
          <BaseButton @click="confirmCheckout">✓ Proses Bayar</BaseButton>
        </div>
      </div>
    </BaseModal>
  </div>
</template>

<script setup>
import { ref, computed } from "vue";
import BaseButton from "../ui/BaseButton.vue";
import BaseModal from "../ui/BaseModal.vue";

const props = defineProps({
  total: { type: Number, default: 0 },
  paidAmount: { type: [Number, String], default: 0 },
  changeAmount: { type: Number, default: 0 },
  canCheckout: { type: Boolean, default: false },
  processing: { type: Boolean, default: false },
  error: { type: String, default: null },
});

const emit = defineEmits(["update:paidAmount", "checkout", "clear"]);

const showConfirm = ref(false);

function requestCheckout() {
  if (!props.canCheckout || props.processing) return;
  showConfirm.value = true;
}

function confirmCheckout() {
  showConfirm.value = false;
  emit("checkout");
}

const quickAmounts = computed(() => {
  const total = props.total || 0;
  const rounded = Math.ceil(total / 1000) * 1000;
  const options = new Set([rounded, rounded + 5000, rounded + 10000, rounded + 50000]);
  return Array.from(options).slice(0, 4);
});

function formatRupiah(value) {
  return new Intl.NumberFormat("id-ID", { style: "currency", currency: "IDR", minimumFractionDigits: 0 }).format(value || 0);
}

function formatRupiahShort(value) {
  if (value >= 1000) return `${(value / 1000).toLocaleString("id-ID")}rb`;
  return formatRupiah(value);
}
</script>