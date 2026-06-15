<template>
  <div class="flex h-screen overflow-hidden">
    <Sidebar />
    <div class="flex-1 flex flex-col overflow-hidden">
      <Header />
      <main class="flex-1 overflow-y-auto p-6">
        <router-view />
      </main>
    </div>
  </div>

  <!-- Toast Global (Fase 6.2) -->
  <ToastContainer />

  <Teleport to="body">
    <!-- Notifikasi Stok Menipis -->
    <Transition name="toast">
      <div
        v-if="showAlert"
        class="fixed bottom-6 left-1/2 -translate-x-1/2 z-50 w-full max-w-sm px-4"
      >
        <div class="bg-white dark:bg-gray-800 border border-yellow-400 shadow-xl rounded-xl p-5">
          <div class="flex items-start gap-3">
            <span class="text-2xl">⚠️</span>
            <div class="flex-1">
              <p class="font-semibold text-gray-800 dark:text-white text-sm">Stok Menipis!</p>
              <ul class="mt-1 space-y-0.5">
                <li
                  v-for="item in lowStockItems"
                  :key="item.id"
                  class="text-xs text-gray-600 dark:text-gray-300"
                >
                  {{ item.name }}
                  <span class="font-semibold" :class="item.stock === 0 ? 'text-red-500' : 'text-yellow-500'">
                    — Sisa {{ item.stock }} {{ item.unit }}
                  </span>
                </li>
              </ul>
            </div>
            <button
              class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-200 text-lg leading-none"
              @click="dismissAlert"
            >✕</button>
          </div>
        </div>
      </div>
    </Transition>

    <!-- Notifikasi Scanner Terdeteksi -->
    <Transition name="toast">
      <div
        v-if="scannerReady"
        class="fixed bottom-6 right-6 z-50"
      >
        <div class="bg-white dark:bg-gray-800 border border-blue-400 shadow-xl rounded-xl px-5 py-3 flex items-center gap-3">
          <span class="text-xl">🔫</span>
          <div>
            <p class="text-sm font-semibold text-gray-800 dark:text-white">Scanner Siap!</p>
            <p class="text-xs text-gray-500 dark:text-gray-400">Barcode scanner terdeteksi dan aktif</p>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup>
import { onMounted } from "vue";
import Sidebar from "./components/layout/Sidebar.vue";
import Header from "./components/layout/Header.vue";
import ToastContainer from "./components/ui/ToastContainer.vue";
import { useNotification } from "./composables/useNotification";
import { scannerReady } from "./composables/useBarcode";
import { useThemeStore } from "./stores/themeStore";

const { showAlert, lowStockItems, checkLowStock, dismissAlert } = useNotification();
const themeStore = useThemeStore();

onMounted(() => {
  themeStore.init();
  checkLowStock();
});
</script>

<style>
.toast-enter-active,
.toast-leave-active {
  transition: all 0.3s ease;
}
.toast-enter-from,
.toast-leave-to {
  opacity: 0;
  transform: translateY(16px);
}
</style>
