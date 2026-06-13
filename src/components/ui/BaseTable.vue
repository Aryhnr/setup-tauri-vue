<template>
  <div class="overflow-x-auto card">
    <table class="table-base">
      <thead>
        <tr>
          <th v-for="col in columns" :key="col.key">{{ col.label }}</th>
          <th v-if="$slots.actions" class="text-right">Aksi</th>
        </tr>
      </thead>
      <tbody>
        <tr v-if="rows.length === 0">
          <td :colspan="columns.length + ($slots.actions ? 1 : 0)" class="text-center text-gray-400 py-6">
            Tidak ada data
          </td>
        </tr>
        <tr v-for="row in rows" :key="row.id" class="hover:bg-gray-50 dark:hover:bg-gray-700/50">
          <td v-for="col in columns" :key="col.key">
            <slot :name="`cell-${col.key}`" :row="row">
              {{ row[col.key] }}
            </slot>
          </td>
          <td v-if="$slots.actions" class="text-right">
            <slot name="actions" :row="row" />
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script setup>
defineProps({
  columns: { type: Array, required: true }, // [{ key, label }]
  rows: { type: Array, default: () => [] },
});
</script>