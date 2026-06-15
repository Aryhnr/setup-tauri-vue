<template>
  <BaseModal :model-value="modelValue" title="Struk Transaksi" @update:model-value="$emit('update:modelValue', $event)">
    <div class="space-y-4">
      <!-- Header toko -->
      <div class="text-center border-b border-dashed border-gray-300 dark:border-gray-700 pb-3">
        <p class="font-black text-base text-gray-900 dark:text-white uppercase tracking-wide">{{ settings.storeName }}</p>
        <p v-if="settings.storeAddress" class="text-xs text-gray-500 mt-0.5">{{ settings.storeAddress }}</p>
        <p v-if="settings.storePhone" class="text-xs text-gray-500">Telp: {{ settings.storePhone }}</p>
      </div>

      <!-- Info transaksi -->
      <div class="text-xs space-y-1 text-gray-600 dark:text-gray-400">
        <div class="flex justify-between">
          <span>No. Invoice</span>
          <span class="font-bold text-gray-900 dark:text-white">{{ transaction?.invoice_no }}</span>
        </div>
        <div class="flex justify-between">
          <span>Tanggal</span>
          <span class="font-medium">{{ formatDate(transaction?.transaction_date) }}</span>
        </div>
      </div>

      <!-- Item-item -->
      <div class="border-t border-dashed border-gray-300 dark:border-gray-700 pt-3 space-y-2">
        <div v-for="item in items" :key="item.id" class="text-xs">
          <p class="font-semibold text-gray-900 dark:text-white">{{ item.item_name }}</p>
          <div class="flex justify-between text-gray-500 dark:text-gray-400">
            <span>{{ item.quantity }} x {{ formatRupiah(item.unit_price) }}</span>
            <span class="font-semibold text-gray-700 dark:text-gray-300">{{ formatRupiah(item.subtotal) }}</span>
          </div>
        </div>
      </div>

      <!-- Total -->
      <div class="border-t border-dashed border-gray-300 dark:border-gray-700 pt-3 space-y-1.5 text-sm">
        <div class="flex justify-between font-black text-gray-900 dark:text-white">
          <span>TOTAL</span>
          <span>{{ formatRupiah(transaction?.total_amount) }}</span>
        </div>
        <div class="flex justify-between text-xs text-gray-500 dark:text-gray-400">
          <span>Bayar</span>
          <span>{{ formatRupiah(transaction?.paid_amount) }}</span>
        </div>
        <div class="flex justify-between text-xs text-gray-500 dark:text-gray-400">
          <span>Kembali</span>
          <span class="font-semibold text-green-600 dark:text-green-400">{{ formatRupiah(transaction?.change_amount) }}</span>
        </div>
      </div>

      <!-- Footer -->
      <div class="border-t border-dashed border-gray-300 dark:border-gray-700 pt-3 text-center">
        <p class="text-xs text-gray-400">Terima kasih telah berbelanja!</p>
      </div>
    </div>

    <!-- Tombol aksi -->
    <div class="flex gap-2 mt-5 pt-4 border-t border-gray-100 dark:border-gray-800">
      <BaseButton variant="secondary" class="flex-1" :disabled="printing" @click="printReceipt">
        <PrinterIcon class="w-4 h-4 mr-1.5 inline" />
        {{ printing ? "Mencetak..." : "Print Struk" }}
      </BaseButton>
      <BaseButton class="flex-1" @click="$emit('update:modelValue', false)">
        Transaksi Baru
      </BaseButton>
    </div>
  </BaseModal>
</template>

<script setup>
import { ref, computed } from "vue";
import BaseModal from "../ui/BaseModal.vue";
import BaseButton from "../ui/BaseButton.vue";
import { PrinterIcon } from "@heroicons/vue/24/outline";
import { useSettingStore } from "../../stores/settingStore";
import { useToast } from "../../composables/useToast";
import { printHtml } from "../../composables/usePrint";

const props = defineProps({
  modelValue: { type: Boolean, default: false },
  transaction: { type: Object, default: null },
  items: { type: Array, default: () => [] },
});

defineEmits(["update:modelValue"]);

const settingStore = useSettingStore();
const settings = computed(() => settingStore.$state);
const { success, error: toastError } = useToast();
const printing = ref(false);

function formatRupiah(value) {
  return new Intl.NumberFormat("id-ID", {
    style: "currency", currency: "IDR", minimumFractionDigits: 0,
  }).format(value || 0);
}

function formatDate(dateStr) {
  if (!dateStr) return "-";
  const d = new Date(dateStr.replace(" ", "T") + "Z");
  return d.toLocaleString("id-ID", {
    day: "numeric", month: "short", year: "numeric",
    hour: "2-digit", minute: "2-digit",
  });
}

async function printReceipt() {
  printing.value = true;
  try {
    const s = settingStore;
    const itemRows = props.items.map((i) =>
      `<div style="margin-bottom:6px">
        <div style="font-weight:600">${i.item_name}</div>
        <div style="display:flex;justify-content:space-between;color:#666">
          <span>${i.quantity} x ${formatRupiah(i.unit_price)}</span>
          <span>${formatRupiah(i.subtotal)}</span>
        </div>
      </div>`
    ).join("");

    const html = `<!DOCTYPE html><html><head><meta charset="utf-8">
    <title>Struk ${props.transaction?.invoice_no}</title>
    <style>
      *{margin:0;padding:0;box-sizing:border-box}
      body{font-family:monospace;font-size:12px;padding:16px;max-width:280px}
      .center{text-align:center}
      .bold{font-weight:700}
      .big{font-size:14px}
      .gray{color:#666}
      .divider{border-top:1px dashed #ccc;margin:8px 0}
      .row{display:flex;justify-content:space-between}
      @media print{body{padding:0}}
    </style></head><body>
    <div class="center bold big">${s.storeName || "Kasir"}</div>
    ${s.storeAddress ? `<div class="center gray">${s.storeAddress}</div>` : ""}
    ${s.storePhone ? `<div class="center gray">Telp: ${s.storePhone}</div>` : ""}
    <div class="divider"></div>
    <div class="row"><span>No. Invoice</span><span class="bold">${props.transaction?.invoice_no}</span></div>
    <div class="row gray"><span>Tanggal</span><span>${formatDate(props.transaction?.transaction_date)}</span></div>
    <div class="divider"></div>
    ${itemRows}
    <div class="divider"></div>
    <div class="row bold"><span>TOTAL</span><span>${formatRupiah(props.transaction?.total_amount)}</span></div>
    <div class="row gray"><span>Bayar</span><span>${formatRupiah(props.transaction?.paid_amount)}</span></div>
    <div class="row gray"><span>Kembali</span><span>${formatRupiah(props.transaction?.change_amount)}</span></div>
    <div class="divider"></div>
    <div class="center gray">Terima kasih telah berbelanja!</div>
    </body></html>`;

    await printHtml(html);
    success("Struk berhasil dicetak!");
  } catch (err) {
    console.error("printReceipt error:", err);
    toastError("Gagal mencetak struk");
  } finally {
    printing.value = false;
  }
}
</script>