<template>
  <div>
    <label v-if="label" class="label">{{ label }}<span v-if="required" class="text-red-500"> *</span></label>
    <input
      :type="type"
      :value="modelValue"
      :placeholder="placeholder"
      :step="step"
      :required="required"
      class="input"
      @input="$emit('update:modelValue', type === 'number' ? Number($event.target.value) : $event.target.value)"
    />
  </div>
</template>

<script setup>
defineProps({
  modelValue: { type: [String, Number], default: "" },
  label: { type: String, default: "" },
  type: { type: String, default: "text" },
  placeholder: { type: String, default: "" },
  step: { type: [String, Number], default: undefined },
  required: { type: Boolean, default: false },
});

defineEmits(["update:modelValue"]);
</script>