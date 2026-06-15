<template>
  <div v-if="totalPages > 1" class="flex items-center justify-between px-1 py-3 select-none">
    <p class="text-xs text-gray-500 dark:text-gray-400">
      Menampilkan <span class="font-semibold text-gray-700 dark:text-gray-300">{{ from }}–{{ to }}</span>
      dari <span class="font-semibold text-gray-700 dark:text-gray-300">{{ total }}</span> data
    </p>
    <div class="flex items-center gap-1">
      <!-- Prev -->
      <button
        class="w-8 h-8 flex items-center justify-center text-xs font-bold rounded border transition-colors"
        :class="modelValue === 1
          ? 'opacity-30 cursor-not-allowed border-gray-200 dark:border-gray-700 text-gray-400'
          : 'border-gray-200 dark:border-gray-700 text-gray-600 dark:text-gray-300 hover:border-blue-400 hover:text-blue-600'"
        :disabled="modelValue === 1"
        @click="$emit('update:modelValue', modelValue - 1)"
      >‹</button>

      <!-- Halaman -->
      <template v-for="p in pageList" :key="p">
        <span v-if="p === '...'" class="w-8 h-8 flex items-center justify-center text-xs text-gray-400">…</span>
        <button
          v-else
          class="w-8 h-8 flex items-center justify-center text-xs font-bold rounded border transition-colors"
          :class="p === modelValue
            ? 'bg-blue-600 text-white border-blue-600'
            : 'bg-white dark:bg-gray-900 border-gray-200 dark:border-gray-700 text-gray-600 dark:text-gray-300 hover:border-blue-400 hover:text-blue-600'"
          @click="$emit('update:modelValue', p)"
        >{{ p }}</button>
      </template>

      <!-- Next -->
      <button
        class="w-8 h-8 flex items-center justify-center text-xs font-bold rounded border transition-colors"
        :class="modelValue === totalPages
          ? 'opacity-30 cursor-not-allowed border-gray-200 dark:border-gray-700 text-gray-400'
          : 'border-gray-200 dark:border-gray-700 text-gray-600 dark:text-gray-300 hover:border-blue-400 hover:text-blue-600'"
        :disabled="modelValue === totalPages"
        @click="$emit('update:modelValue', modelValue + 1)"
      >›</button>
    </div>
  </div>
</template>

<script setup>
import { computed } from "vue";

const props = defineProps({
  modelValue: { type: Number, default: 1 },   // halaman aktif
  total: { type: Number, required: true },      // total baris data
  perPage: { type: Number, default: 20 },       // baris per halaman
});

defineEmits(["update:modelValue"]);

const totalPages = computed(() => Math.max(1, Math.ceil(props.total / props.perPage)));

const from = computed(() => Math.min((props.modelValue - 1) * props.perPage + 1, props.total));
const to = computed(() => Math.min(props.modelValue * props.perPage, props.total));

// Smart page list: tampilkan maks 7 tombol dengan ellipsis
const pageList = computed(() => {
  const total = totalPages.value;
  const cur = props.modelValue;
  if (total <= 7) return Array.from({ length: total }, (_, i) => i + 1);

  const pages = [];
  pages.push(1);
  if (cur > 3) pages.push("...");
  for (let p = Math.max(2, cur - 1); p <= Math.min(total - 1, cur + 1); p++) {
    pages.push(p);
  }
  if (cur < total - 2) pages.push("...");
  pages.push(total);
  return pages;
});
</script>