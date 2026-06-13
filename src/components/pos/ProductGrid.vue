<template>
  <div class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 gap-3">
    <button
      v-for="product in products"
      :key="product.id"
      class="card p-3 text-left hover:border-blue-400 hover:shadow-md transition-all disabled:opacity-40 disabled:cursor-not-allowed"
      :disabled="product.stock <= 0"
      @click="$emit('add', product)"
    >
      <div class="aspect-square rounded-lg bg-gray-100 dark:bg-gray-700 mb-2 flex items-center justify-center text-3xl">
        📦
      </div>
      <p class="text-sm font-medium truncate">{{ product.name }}</p>
      <p class="text-xs text-gray-400">{{ product.category_name || "-" }}</p>
      <div class="flex items-center justify-between mt-1">
        <p class="text-sm font-semibold text-blue-600 dark:text-blue-400">
          {{ formatRupiah(product.sell_price) }}
        </p>
        <span
          class="text-xs px-1.5 py-0.5 rounded-full"
          :class="product.stock <= product.min_stock
            ? 'bg-red-100 text-red-600 dark:bg-red-900/40 dark:text-red-300'
            : 'bg-green-100 text-green-600 dark:bg-green-900/40 dark:text-green-300'"
        >
          {{ product.stock }} {{ product.unit }}
        </span>
      </div>
    </button>

    <p v-if="products.length === 0" class="col-span-full text-center text-gray-400 py-10">
      Produk tidak ditemukan
    </p>
  </div>
</template>

<script setup>
defineProps({
  products: { type: Array, default: () => [] },
});

defineEmits(["add"]);

function formatRupiah(value) {
  return new Intl.NumberFormat("id-ID", { style: "currency", currency: "IDR", minimumFractionDigits: 0 }).format(
    value || 0
  );
}
</script>