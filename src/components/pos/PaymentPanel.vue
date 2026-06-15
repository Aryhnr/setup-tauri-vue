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

    <!-- Info sisa hutang kalau bayar kurang -->
    <div
      v-if="debtAmount > 0"
      class="flex items-center justify-between text-sm bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-900/40 rounded-lg px-3 py-2"
    >
      <span class="text-red-600 dark:text-red-400 font-semibold">Sisa jadi hutang</span>
      <span class="font-black text-red-600 dark:text-red-400">{{ formatRupiah(debtAmount) }}</span>
    </div>

    <p v-if="error" class="text-sm text-red-500">{{ error }}</p>

    <div class="grid grid-cols-2 gap-2 pt-1">
      <BaseButton variant="secondary" :disabled="processing" @click="$emit('clear')">Batal</BaseButton>
      <BaseButton
        data-pay-btn
        :disabled="!canCheckout || processing"
        @click="requestCheckout"
      >
        {{ processing ? "Memproses..." : "Bayar" }}
      </BaseButton>
    </div>

    <!-- Tombol Hutang -->
    <BaseButton
      variant="secondary"
      class="w-full flex items-center justify-center gap-2 border-dashed border-red-300 dark:border-red-800 text-red-600 dark:text-red-400 hover:bg-red-50 dark:hover:bg-red-900/20"
      :disabled="processing || total <= 0"
      @click="openDebtModal"
    >
      <BanknotesIcon class="w-4 h-4" />
      {{ debtAmount > 0 ? `Bayar ${formatRupiahShort(Number(paidAmount))} + Hutang ${formatRupiahShort(debtAmount)}` : 'Catat sebagai Hutang' }}
    </BaseButton>

    <!-- Modal Konfirmasi Bayar -->
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

    <!-- Modal Catat Hutang -->
    <BaseModal v-model="showDebtModal" title="Catat Hutang Pelanggan">
      <div class="space-y-4">
        <div class="bg-gray-50 dark:bg-gray-800 rounded-lg p-3 space-y-1.5 text-sm">
          <div class="flex justify-between">
            <span class="text-gray-500">Total Belanja</span>
            <span class="font-bold text-gray-900 dark:text-white">{{ formatRupiah(total) }}</span>
          </div>
          <div class="flex justify-between">
            <span class="text-gray-500">Dibayar Sekarang</span>
            <span class="font-semibold text-green-600 dark:text-green-400">
              {{ formatRupiah(Number(paidAmount) || 0) }}
            </span>
          </div>
          <div class="border-t border-gray-200 dark:border-gray-700 pt-1.5 flex justify-between">
            <span class="font-semibold text-red-600 dark:text-red-400">Dicatat sebagai Hutang</span>
            <span class="font-black text-red-600 dark:text-red-400">
              {{ formatRupiah(debtAmount > 0 ? debtAmount : total) }}
            </span>
          </div>
        </div>

        <form class="space-y-3" @submit.prevent="confirmDebt">
          <BaseInput
            v-model="debtForm.name"
            label="Nama Pelanggan"
            required
            placeholder="Budi Santoso"
          />
          <BaseInput
            v-model="debtForm.phone"
            label="Nomor HP (opsional)"
            placeholder="08xxxxxxxxxx"
          />
          <BaseInput
            v-model="debtForm.due_date"
            type="date"
            label="Jatuh Tempo (opsional)"
          />
          <div>
            <label class="block text-xs font-bold text-gray-700 dark:text-gray-300 uppercase tracking-wider mb-1.5">Keterangan</label>
            <textarea
              v-model="debtForm.notes"
              class="w-full bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-md px-3 py-2 text-sm font-medium text-gray-900 dark:text-white focus:outline-none focus:border-blue-500 transition-colors resize-none"
              rows="2"
              placeholder="Keterangan hutang..."
            ></textarea>
          </div>
          <div class="grid grid-cols-2 gap-2 pt-3 border-t border-gray-100 dark:border-gray-800">
            <BaseButton type="button" variant="secondary" @click="showDebtModal = false">Batal</BaseButton>
            <BaseButton type="submit" :disabled="!debtForm.name.trim()">
              Proses & Catat Hutang
            </BaseButton>
          </div>
        </form>
      </div>
    </BaseModal>
  </div>
</template>

<script setup>
import { ref, reactive, computed, onMounted, onUnmounted } from "vue";
import BaseButton from "../ui/BaseButton.vue";
import BaseModal from "../ui/BaseModal.vue";
import BaseInput from "../ui/BaseInput.vue";
import { BanknotesIcon } from "@heroicons/vue/24/outline";

const props = defineProps({
  total: { type: Number, default: 0 },
  paidAmount: { type: [Number, String], default: 0 },
  changeAmount: { type: Number, default: 0 },
  canCheckout: { type: Boolean, default: false },
  processing: { type: Boolean, default: false },
  error: { type: String, default: null },
});

const emit = defineEmits(["update:paidAmount", "checkout", "checkout-debt", "clear"]);

const showConfirm = ref(false);
const showDebtModal = ref(false);

const debtForm = reactive({
  name: "",
  phone: "",
  due_date: "",
  notes: "",
});

// Dengarkan event Escape dari PosView
// Kalau ada modal yang terbuka → tutup dan beri tahu PosView bahwa sudah ditangani
// Kalau tidak ada → dispatch "pos:escape:unhandled" agar PosView lanjut handle
function handleEscapeEvent() {
  if (showConfirm.value) {
    showConfirm.value = false;
    return; // handled — jangan dispatch unhandled
  }
  if (showDebtModal.value) {
    showDebtModal.value = false;
    return; // handled
  }
  // Tidak ada modal di PaymentPanel yang terbuka → lempar balik ke PosView
  window.dispatchEvent(new CustomEvent("pos:escape:unhandled"));
}

onMounted(() => window.addEventListener("pos:escape", handleEscapeEvent));
onUnmounted(() => window.removeEventListener("pos:escape", handleEscapeEvent));

const debtAmount = computed(() => {
  const paid = Number(props.paidAmount) || 0;
  const sisa = props.total - paid;
  return sisa > 0 ? sisa : 0;
});

function requestCheckout() {
  if (!props.canCheckout || props.processing) return;
  showConfirm.value = true;
}

function confirmCheckout() {
  showConfirm.value = false;
  emit("checkout");
}

function openDebtModal() {
  if (props.total <= 0) return;
  Object.assign(debtForm, { name: "", phone: "", due_date: "", notes: "" });
  showDebtModal.value = true;
}

function confirmDebt() {
  if (!debtForm.name.trim()) return;
  showDebtModal.value = false;
  emit("checkout-debt", {
    name: debtForm.name,
    phone: debtForm.phone || null,
    due_date: debtForm.due_date || null,
    notes: debtForm.notes || null,
    paid_upfront: Number(props.paidAmount) || 0,
    debt_amount: debtAmount.value > 0 ? debtAmount.value : props.total,
  });
}

const quickAmounts = computed(() => {
  const total = props.total || 0;
  const rounded = Math.ceil(total / 1000) * 1000;
  const options = new Set([rounded, rounded + 5000, rounded + 10000, rounded + 50000]);
  return Array.from(options).slice(0, 4);
});

function formatRupiah(value) {
  return new Intl.NumberFormat("id-ID", {
    style: "currency",
    currency: "IDR",
    minimumFractionDigits: 0,
  }).format(value || 0);
}

function formatRupiahShort(value) {
  if (value >= 1000) return `${(value / 1000).toLocaleString("id-ID")}rb`;
  return formatRupiah(value);
}
</script>