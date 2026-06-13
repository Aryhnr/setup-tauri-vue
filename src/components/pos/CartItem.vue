<template>
  <div class="flex items-center gap-2 py-2 border-b border-gray-100 dark:border-gray-700 last:border-b-0">
    <div class="flex-1 min-w-0">
      <p class="text-sm font-medium truncate">{{ item.item_name }}</p>
      <p class="text-xs text-gray-400">{{ formatRupiah(item.unit_price) }} / {{ item.unit || "item" }}</p>
    </div>

    <div class="flex items-center gap-1">
      <button
        class="w-6 h-6 rounded bg-gray-100 dark:bg-gray-700 text-sm font-bold hover:bg-gray-200 dark:hover:bg-gray-600"
        @click="$emit('update-quantity', item.quantity - 1)"
      >
        −
      </button>
      <input
        type="number"
        class="w-12 text-center text-sm border border-gray-200 dark:border-gray-600 rounded bg-white dark:bg-gray-700 py-0.5"
        :value="item.quantity"
        min="1"
        @change="$emit('update-quantity', $event.target.value)"
      />
      <button
        class="w-6 h-6 rounded bg-gray-100 dark:bg-gray-700 text-sm font-bold hover:bg-gray-200 dark:hover:bg-gray-600"
        @click="$emit('update-quantity', item.quantity + 1)"
      >
        +
      </button>
    </div>

    <p class="w-24 text-right text-sm font-semibold">{{ formatRupiah(item.subtotal) }}</p>

    <button class="text-red-400 hover:text-red-600 text-sm px-1" @click="$emit('remove')">✕</button>
  </div>
</template>

<script setup>
defineProps({
  item: { type: Object, required: true },
});

defineEmits(["update-quantity", "remove"]);

function formatRupiah(value) {
  return new Intl.NumberFormat("id-ID", { style: "currency", currency: "IDR", minimumFractionDigits: 0 }).format(
    value || 0
  );
}
</script>