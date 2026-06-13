<template>
  <button :type="type" :class="classes" :disabled="disabled" @click="$emit('click', $event)">
    <slot />
  </button>
</template>

<script setup>
import { computed } from "vue";

const props = defineProps({
  variant: { type: String, default: "primary" }, // primary | secondary | danger
  type: { type: String, default: "button" },
  disabled: { type: Boolean, default: false },
});

defineEmits(["click"]);

const classes = computed(() => {
  switch (props.variant) {
    case "secondary":
      return "btn-secondary";
    case "danger":
      return "btn-danger";
    default:
      return "btn-primary";
  }
});
</script>