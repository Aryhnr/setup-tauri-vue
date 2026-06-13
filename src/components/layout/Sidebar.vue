<template>
  <aside class="w-64 flex-shrink-0 bg-white dark:bg-gray-800 border-r border-gray-200 dark:border-gray-700 flex flex-col">
    <div class="px-5 py-5 border-b border-gray-200 dark:border-gray-700">
      <h1 class="text-lg font-bold text-blue-600 dark:text-blue-400">🏪 {{ storeName }}</h1>
      <p class="text-xs text-gray-400 mt-0.5">Sistem Manajemen Toko</p>
    </div>

    <nav class="flex-1 overflow-y-auto py-3 px-2 space-y-1">
      <router-link
        v-for="item in menu"
        :key="item.path"
        :to="item.path"
        class="flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm font-medium transition-colors"
        :class="isActive(item.path)
          ? 'bg-blue-600 text-white'
          : 'text-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700'"
      >
        <component :is="item.icon" class="w-5 h-5" />
        <span>{{ item.label }}</span>
      </router-link>
    </nav>

    <div class="p-3 border-t border-gray-200 dark:border-gray-700">
      <button class="btn-secondary w-full" @click="toggleDark">
        <span v-if="isDark">☀️ Light Mode</span>
        <span v-else>🌙 Dark Mode</span>
      </button>
    </div>
  </aside>
</template>

<script setup>
import { ref, computed, onMounted } from "vue";
import { useRoute } from "vue-router";
import {
  HomeIcon,
  ShoppingCartIcon,
  CubeIcon,
  PrinterIcon,
  BoltIcon,
  TruckIcon,
  ChartBarIcon,
  CogIcon,
} from "@heroicons/vue/24/outline";

const route = useRoute();
const storeName = ref("Toko Madura");
const isDark = ref(false);

const menu = [
  { path: "/", label: "Dashboard", icon: HomeIcon },
  { path: "/kasir", label: "Kasir / POS", icon: ShoppingCartIcon },
  { path: "/produk", label: "Produk & Stok", icon: CubeIcon },
  { path: "/percetakan", label: "Percetakan", icon: PrinterIcon },
  { path: "/jasa", label: "Jasa & Layanan", icon: BoltIcon },
  { path: "/supplier", label: "Supplier", icon: TruckIcon },
  { path: "/laporan", label: "Laporan", icon: ChartBarIcon },
  { path: "/pengaturan", label: "Pengaturan", icon: CogIcon },
];

function isActive(path) {
  return route.path === path;
}

function toggleDark() {
  isDark.value = !isDark.value;
  document.documentElement.classList.toggle("dark", isDark.value);
  localStorage.setItem("theme", isDark.value ? "dark" : "light");
}

onMounted(() => {
  const saved = localStorage.getItem("theme");
  isDark.value = saved === "dark";
  document.documentElement.classList.toggle("dark", isDark.value);
});
</script>