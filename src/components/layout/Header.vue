<template>
  <header class="h-16 flex items-center justify-between px-6 bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
    <h2 class="text-lg font-semibold">{{ title }}</h2>
    <div class="text-sm text-gray-400">{{ today }}</div>
  </header>
</template>

<script setup>
import { computed } from "vue";
import { useRoute } from "vue-router";

const route = useRoute();
const title = computed(() => route.meta?.title || "");

const today = new Date().toLocaleDateString("id-ID", {
  weekday: "long",
  day: "numeric",
  month: "long",
  year: "numeric",
});
</script>