<template>
  <div class="card p-4">
    <h3 class="font-semibold mb-3">{{ title }}</h3>

    <div v-if="rows.length === 0" class="text-center text-gray-400 text-sm py-6">
      Tidak ada data
    </div>

    <table v-else class="table-base">
      <thead>
        <tr>
          <th class="w-8">#</th>
          <th>Produk</th>
          <th class="text-right">Terjual</th>
          <th class="text-right">Omset</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="(row, index) in rows" :key="row.product_id ?? row.name" class="hover:bg-gray-50 dark:hover:bg-gray-700/50">
          <td class="text-gray-400">{{ index + 1 }}</td>
          <td class="font-medium">{{ row.name }}</td>
          <td class="text-right">{{ row.total_quantity }}</td>
          <td class="text-right">{{ formatRupiah(row.total_revenue) }}</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script setup>
defineProps({
  title: { type: String, default: "Produk Terlaris" },
  rows: { type: Array, default: () => [] },
});

function formatRupiah(value) {
  return new Intl.NumberFormat("id-ID", { style: "currency", currency: "IDR", minimumFractionDigits: 0 }).format(
    value || 0
  );
}
</script>