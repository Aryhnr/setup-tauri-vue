<template>
  <header class="h-16 flex items-center justify-between px-6 bg-white dark:bg-gray-900 border-b border-gray-200 dark:border-gray-800 flex-shrink-0 select-none">
    
    <div class="flex items-center">
      <h2 class="text-xl font-bold text-gray-900 dark:text-white tracking-tight">
        {{ title }}
      </h2>
    </div>

    <div class="flex items-center gap-2 px-3 py-1.5 bg-gray-50 dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-md">
      <CalendarIcon class="w-4 h-4 text-gray-500 dark:text-gray-400" />
      <span class="text-sm font-medium text-gray-600 dark:text-gray-300">
        {{ today }}
      </span>
    </div>

  </header>
</template>

<script setup>
import { computed } from "vue";
import { useRoute } from "vue-router";
import { CalendarIcon } from "@heroicons/vue/24/outline";

const route = useRoute();
// Menambahkan fallback text jika meta.title kosong
const title = computed(() => route.meta?.title || "Dashboard");

const today = new Date().toLocaleDateString("id-ID", {
  weekday: "long",
  day: "numeric",
  month: "long",
  year: "numeric",
});
</script>