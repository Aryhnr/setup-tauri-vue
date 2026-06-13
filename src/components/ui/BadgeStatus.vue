<template>
  <span class="px-2 py-0.5 rounded-full text-xs font-semibold whitespace-nowrap" :class="classes">
    {{ label }}
  </span>
</template>

<script setup>
import { computed } from "vue";

const props = defineProps({
  status: { type: String, required: true }, // ANTRIAN / DIPROSES / SELESAI / DIBAYAR
});

const STATUS_MAP = {
  ANTRIAN: {
    label: "Antrian",
    classes: "bg-gray-100 text-gray-600 dark:bg-gray-700 dark:text-gray-300",
  },
  DIPROSES: {
    label: "Diproses",
    classes: "bg-yellow-100 text-yellow-700 dark:bg-yellow-900/40 dark:text-yellow-300",
  },
  SELESAI: {
    label: "Selesai",
    classes: "bg-blue-100 text-blue-700 dark:bg-blue-900/40 dark:text-blue-300",
  },
  DIBAYAR: {
    label: "Dibayar",
    classes: "bg-green-100 text-green-700 dark:bg-green-900/40 dark:text-green-300",
  },
};

const label = computed(() => STATUS_MAP[props.status]?.label ?? props.status);
const classes = computed(() => STATUS_MAP[props.status]?.classes ?? "bg-gray-100 text-gray-600");
</script>