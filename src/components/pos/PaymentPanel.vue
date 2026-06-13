<template>
  <div class="space-y-3">
    <div class="flex items-center justify-between text-lg font-bold">
      <span>Total</span>
      <span>{{ formatRupiah(total) }}</span>
    </div>

    <div>
      <label class="label">Jumlah Bayar</label>
      <input
        type="number"
        class="input text-lg font-semibold"
        :value="paidAmount"
        placeholder="0"
        @input="$emit('update:paidAmount', $event.target.value)"
      />
    </div>

    <!-- Tombol nominal cepat -->
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
      <BaseButton :disabled="!canCheckout || processing" @click="$emit('checkout')">
        {{ processing ? "Memproses..." : "Bayar" }}
      </BaseButton>
    </div>
  </div>
</template>

<script setup>
import { computed } from "vue";
import BaseButton from "../ui/BaseButton.vue";

const props = defineProps({
  total: { type: Number, default: 0 },
  paidAmount: { type: [Number, String], default: 0 },
  changeAmount: { type: Number, default: 0 },
  canCheckout: { type: Boolean, default: false },
  processing: { type: Boolean, default: false },
  error: { type: String, default: null },
});

defineEmits(["update:paidAmount", "checkout", "clear"]);

const quickAmounts = computed(() => {
  const total = props.total || 0;
  const rounded = Math.ceil(total / 1000) * 1000;
  const options = new Set([rounded, rounded + 5000, rounded + 10000, rounded + 50000]);
  return Array.from(options).slice(0, 4);
});

function formatRupiah(value) {
  return new Intl.NumberFormat("id-ID", { style: "currency", currency: "IDR", minimumFractionDigits: 0 }).format(
    value || 0
  );
}

function formatRupiahShort(value) {
  if (value >= 1000) {
    return `${(value / 1000).toLocaleString("id-ID")}rb`;
  }
  return formatRupiah(value);
}
</script>