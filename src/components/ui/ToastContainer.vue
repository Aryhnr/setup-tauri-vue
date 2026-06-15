<template>
  <Teleport to="body">
    <div class="fixed top-5 right-5 z-[9999] flex flex-col gap-2 pointer-events-none" aria-live="polite">
      <TransitionGroup name="toast">
        <div
          v-for="toast in toasts"
          :key="toast.id"
          class="pointer-events-auto flex items-start gap-3 px-4 py-3 rounded-lg shadow-lg border max-w-sm w-full text-sm font-medium"
          :class="styleMap[toast.type]"
        >
          <component :is="iconMap[toast.type]" class="w-4 h-4 flex-shrink-0 mt-0.5" />
          <span class="flex-1 leading-snug">{{ toast.message }}</span>
          <button
            class="opacity-60 hover:opacity-100 transition-opacity"
            @click="removeToast(toast.id)"
          >
            <XMarkIcon class="w-4 h-4" />
          </button>
        </div>
      </TransitionGroup>
    </div>
  </Teleport>
</template>

<script setup>
import {
  CheckCircleIcon,
  XCircleIcon,
  InformationCircleIcon,
  ExclamationTriangleIcon,
  XMarkIcon,
} from "@heroicons/vue/24/outline";
import { useToast } from "../../composables/useToast";

const { toasts, removeToast } = useToast();

const styleMap = {
  success: "bg-green-50 dark:bg-green-900/30 border-green-200 dark:border-green-800 text-green-800 dark:text-green-200",
  error:   "bg-red-50 dark:bg-red-900/30 border-red-200 dark:border-red-800 text-red-800 dark:text-red-200",
  info:    "bg-blue-50 dark:bg-blue-900/30 border-blue-200 dark:border-blue-800 text-blue-800 dark:text-blue-200",
  warning: "bg-yellow-50 dark:bg-yellow-900/30 border-yellow-200 dark:border-yellow-800 text-yellow-800 dark:text-yellow-200",
};

const iconMap = {
  success: CheckCircleIcon,
  error:   XCircleIcon,
  info:    InformationCircleIcon,
  warning: ExclamationTriangleIcon,
};
</script>

<style scoped>
.toast-enter-active,
.toast-leave-active {
  transition: all 0.3s ease;
}
.toast-enter-from {
  opacity: 0;
  transform: translateX(24px);
}
.toast-leave-to {
  opacity: 0;
  transform: translateX(24px);
}
.toast-move {
  transition: transform 0.3s ease;
}
</style>