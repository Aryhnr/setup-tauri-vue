<template>
  <div class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 gap-3">
    <button
      v-for="product in products"
      :key="product.id"
      class="card p-3 text-left border border-gray-200 dark:border-gray-800 hover:border-gray-400 dark:hover:border-gray-600 transition-colors disabled:opacity-40 disabled:cursor-not-allowed"
      :disabled="product.stock <= 0"
      @click="$emit('add', product)"
    >
      <!-- Thumbnail: barcode SVG jika ada, ikon box jika tidak -->
      <div class="aspect-square rounded-lg bg-gray-50 dark:bg-gray-800 mb-2 flex items-center justify-center border border-gray-100 dark:border-gray-700/50 overflow-hidden p-1">
        <svg
          v-if="product.barcode"
          :viewBox="`0 0 ${barcodeViewWidth(product.barcode)} 48`"
          xmlns="http://www.w3.org/2000/svg"
          class="w-full h-full"
          preserveAspectRatio="xMidYMid meet"
        >
          <!-- Batang barcode dari karakter barcode -->
          <template v-for="(bar, i) in barcodeStripes(product.barcode)" :key="i">
            <rect
              :x="bar.x"
              y="4"
              :width="bar.w"
              height="36"
              :fill="bar.dark ? 'currentColor' : 'transparent'"
              class="text-gray-800 dark:text-gray-200"
            />
          </template>
          <!-- Teks barcode di bawah -->
          <text
            :x="barcodeViewWidth(product.barcode) / 2"
            y="47"
            text-anchor="middle"
            font-size="5"
            class="fill-gray-600 dark:fill-gray-400"
            font-family="monospace"
          >{{ product.barcode }}</text>
        </svg>

        <!-- Fallback ikon jika tidak ada barcode -->
        <svg
          v-else
          xmlns="http://www.w3.org/2000/svg"
          class="w-8 h-8 text-gray-400 dark:text-gray-500"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="1.5"
          stroke-linecap="round"
          stroke-linejoin="round"
        >
          <path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z" />
          <polyline points="3.27 6.96 12 12.01 20.73 6.96" />
          <line x1="12" y1="22.08" x2="12" y2="12" />
        </svg>
      </div>

      <p class="text-sm font-medium text-gray-800 dark:text-gray-200 truncate">{{ product.name }}</p>
      <p class="text-xs text-gray-400 dark:text-gray-500 mb-1">{{ product.category_name || "-" }}</p>

      <div class="flex items-center justify-between mt-auto pt-1">
        <p class="text-sm font-semibold text-blue-600 dark:text-blue-400">
          {{ formatRupiah(product.sell_price) }}
        </p>
        <span
          class="text-xs px-2 py-0.5 rounded border"
          :class="product.stock <= product.min_stock
            ? 'bg-red-50 text-red-600 border-red-200 dark:bg-red-950/20 dark:text-red-400 dark:border-red-900/50'
            : 'bg-green-50 text-green-600 border-green-200 dark:bg-green-950/20 dark:text-green-400 dark:border-green-900/50'"
        >
          {{ product.stock }} {{ product.unit }}
        </span>
      </div>
    </button>

    <p v-if="products.length === 0" class="col-span-full text-center text-gray-400 dark:text-gray-500 py-10 text-sm">
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
  return new Intl.NumberFormat("id-ID", { style: "currency", currency: "IDR", minimumFractionDigits: 0 }).format(value || 0);
}

/**
 * Generate barcode stripe sederhana dari string barcode.
 * Setiap karakter di-hash jadi pola gelap/terang.
 * Ini visual representasi — bukan standar EAN/Code128.
 */
function barcodeStripes(code) {
  const bars = [];
  let x = 2;
  const stripeW = 2;
  const gapW = 1;

  // Quiet zone kiri
  x += 4;

  for (let i = 0; i < code.length; i++) {
    const charCode = code.charCodeAt(i);
    // Tiap karakter hasilkan 4 batang dengan lebar bervariasi
    const pattern = [
      ((charCode >> 5) & 1) + 1,
      ((charCode >> 4) & 1) + 1,
      ((charCode >> 3) & 1) + 2,
      ((charCode >> 2) & 1) + 1,
    ];
    for (let j = 0; j < pattern.length; j++) {
      const w = pattern[j] * stripeW;
      bars.push({ x, w, dark: j % 2 === 0 });
      x += w + gapW;
    }
    x += 1; // jarak antar karakter
  }

  // Quiet zone kanan
  x += 4;

  return bars;
}

function barcodeViewWidth(code) {
  // Estimasi lebar total: 4 + (10 karakter × (4 batang × rata2 1.5×2 + gap) + 1) + 4
  return 8 + code.length * 14 + 8;
}
</script>