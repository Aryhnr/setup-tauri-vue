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
        <tr
          v-for="row in rows"
          :key="row.id"
          class="hover:bg-gray-50 dark:hover:bg-gray-700/50"
          :class="{ 'cursor-pointer': hasRowClick }"
          @click="emit('row-click', row)"
        >
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
import { useAttrs } from "vue";

defineProps({
  columns: { type: Array, required: true },
  rows: { type: Array, default: () => [] },
});

const emit = defineEmits(["row-click"]);

// Cek apakah parent pasang listener row-click, untuk kasih cursor-pointer
const attrs = useAttrs();
const hasRowClick = "onRow-click" in attrs;
</script>