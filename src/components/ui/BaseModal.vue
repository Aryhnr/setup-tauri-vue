<template>
  <Teleport to="body">
    <div v-if="modelValue" class="fixed inset-0 z-50 flex items-center justify-center bg-black/40 p-4">
      <div class="card w-full max-w-lg p-6 max-h-[90vh] overflow-y-auto">
        <div class="flex items-center justify-between mb-4">
          <h3 class="text-lg font-semibold">{{ title }}</h3>
          <button class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-200" @click="close">✕</button>
        </div>
        <slot />
      </div>
    </div>
  </Teleport>
</template>

<script setup>
const props = defineProps({
  modelValue: { type: Boolean, default: false },
  title: { type: String, default: "" },
});

const emit = defineEmits(["update:modelValue"]);

function close() {
  emit("update:modelValue", false);
}
</script>