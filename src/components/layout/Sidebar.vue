<template>
  <aside class="w-64 h-screen flex-shrink-0 bg-white dark:bg-gray-900 border-r border-gray-200 dark:border-gray-800 flex flex-col select-none">
    <div class="px-6 py-5 border-b border-gray-200 dark:border-gray-800 flex items-center gap-3">
      <div class="p-2 bg-blue-600 rounded-md">
        <BuildingStorefrontIcon class="w-6 h-6 text-white" />
      </div>
      <div>
        <h1 class="text-base font-bold text-gray-900 dark:text-white tracking-wide uppercase">{{ storeName }}</h1>
        <p class="text-xs font-medium text-gray-500 dark:text-gray-400 mt-0.5">Sistem Manajemen Toko</p>
      </div>
    </div>

    <nav class="flex-1 overflow-y-auto py-4 px-3 space-y-1">
      <router-link
        v-for="item in menu"
        :key="item.path"
        :to="item.path"
        class="flex items-center gap-3 px-3 py-2.5 rounded-md text-sm font-medium transition-colors border"
        :class="isActive(item.path)
          ? 'bg-blue-50 border-blue-200 text-blue-700 dark:bg-blue-900/20 dark:border-blue-800 dark:text-blue-400'
          : 'border-transparent text-gray-600 dark:text-gray-400 hover:bg-gray-50 dark:hover:bg-gray-800 hover:text-gray-900 dark:hover:text-white'"
      >
        <component :is="item.icon" class="w-5 h-5" />
        <span>{{ item.label }}</span>
      </router-link>
    </nav>

    <div class="p-4 border-t border-gray-200 dark:border-gray-800">
      <button
        class="flex items-center justify-center gap-2 w-full px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-900 border border-gray-300 dark:border-gray-700 rounded-md hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors"
        @click="themeStore.toggle()"
      >
        <SunIcon v-if="themeStore.isDark" class="w-5 h-5" />
        <MoonIcon v-else class="w-5 h-5" />
        <span>{{ themeStore.isDark ? 'Light Mode' : 'Dark Mode' }}</span>
      </button>
    </div>
  </aside>
</template>

<script setup>
import { ref, onMounted } from "vue";
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
  BuildingStorefrontIcon,
  SunIcon,
  MoonIcon,
  ArrowDownTrayIcon,
  ClipboardDocumentListIcon,
  BanknotesIcon,
  CalculatorIcon,
} from "@heroicons/vue/24/outline";
import { useThemeStore } from "../../stores/themeStore";
import { useSettingStore } from "../../stores/settingStore";

const route = useRoute();
const themeStore = useThemeStore();
const settingStore = useSettingStore();

const storeName = ref("Toko Madura");

const menu = [
  { path: "/", label: "Dashboard", icon: HomeIcon },
  { path: "/kasir", label: "Kasir / POS", icon: ShoppingCartIcon },
  { path: "/transaksi", label: "Riwayat Transaksi", icon: ClipboardDocumentListIcon },
  { path: "/produk", label: "Produk", icon: CubeIcon },
  { path: "/stok", label: "Manajemen Stok", icon: ArrowDownTrayIcon },
  { path: "/percetakan", label: "Percetakan", icon: PrinterIcon },
  { path: "/jasa", label: "Jasa & Layanan", icon: BoltIcon },
  { path: "/hutang", label: "Hutang Pelanggan", icon: BanknotesIcon },
  { path: "/supplier", label: "Supplier", icon: TruckIcon },
  { path: "/rekap-kasir", label: "Rekap Kasir", icon: CalculatorIcon },
  { path: "/laporan", label: "Laporan", icon: ChartBarIcon },
  { path: "/pengaturan", label: "Pengaturan", icon: CogIcon },
];

function isActive(path) {
  return route.path === path;
}

onMounted(() => {
  storeName.value = settingStore.storeName || "Toko Madura";
});
</script>
