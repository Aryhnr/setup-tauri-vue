<template>
  <Transition name="calc-slide">
    <div
      v-if="show"
      class="bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-xl shadow-xl p-4 w-64 select-none"
    >
      <div class="flex items-center justify-between mb-3">
        <p class="text-xs font-black text-gray-500 dark:text-gray-400 uppercase tracking-wider">Kalkulator</p>
        <button class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-200" @click="$emit('close')">
          <XMarkIcon class="w-4 h-4" />
        </button>
      </div>

      <!-- Display -->
      <div class="bg-gray-50 dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg px-3 py-2 mb-3 text-right">
        <p class="text-xs text-gray-400 h-4 truncate">{{ expression || ' ' }}</p>
        <p class="text-xl font-black text-gray-900 dark:text-white font-mono truncate">{{ display }}</p>
      </div>

      <!-- Tombol -->
      <div class="grid grid-cols-4 gap-1.5">
        <button
          v-for="btn in buttons"
          :key="btn.label"
          class="h-10 rounded-lg text-sm font-bold transition-colors"
          :class="btn.class"
          @click="handleButton(btn)"
        >
          {{ btn.label }}
        </button>
      </div>

      <!-- Tombol pindah ke bayar -->
      <button
        class="mt-3 w-full py-2 text-sm font-bold bg-blue-600 hover:bg-blue-700 text-white rounded-lg transition-colors flex items-center justify-center gap-2"
        @click="sendToBayar"
      >
        <ArrowRightIcon class="w-4 h-4" />
        Gunakan sebagai Jumlah Bayar
      </button>
    </div>
  </Transition>
</template>

<script setup>
import { ref, computed } from "vue";
import { XMarkIcon, ArrowRightIcon } from "@heroicons/vue/24/outline";

defineProps({ show: { type: Boolean, default: false } });
const emit = defineEmits(["close", "send-to-bayar"]);

const expression = ref("");
const currentInput = ref("0");
const justCalculated = ref(false);

const display = computed(() => {
  const num = parseFloat(currentInput.value);
  if (isNaN(num)) return currentInput.value;
  return num.toLocaleString("id-ID");
});

const buttons = [
  { label: "C", action: "clear", class: "bg-red-50 dark:bg-red-900/20 text-red-600 dark:text-red-400 hover:bg-red-100 dark:hover:bg-red-900/40" },
  { label: "±", action: "negate", class: "bg-gray-100 dark:bg-gray-800 text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-700" },
  { label: "%", action: "percent", class: "bg-gray-100 dark:bg-gray-800 text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-700" },
  { label: "÷", action: "operator", value: "/", class: "bg-blue-100 dark:bg-blue-900/30 text-blue-600 dark:text-blue-400 hover:bg-blue-200 dark:hover:bg-blue-900/50" },

  { label: "7", action: "digit", value: "7", class: "bg-white dark:bg-gray-800 text-gray-900 dark:text-white border border-gray-200 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-700" },
  { label: "8", action: "digit", value: "8", class: "bg-white dark:bg-gray-800 text-gray-900 dark:text-white border border-gray-200 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-700" },
  { label: "9", action: "digit", value: "9", class: "bg-white dark:bg-gray-800 text-gray-900 dark:text-white border border-gray-200 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-700" },
  { label: "×", action: "operator", value: "*", class: "bg-blue-100 dark:bg-blue-900/30 text-blue-600 dark:text-blue-400 hover:bg-blue-200 dark:hover:bg-blue-900/50" },

  { label: "4", action: "digit", value: "4", class: "bg-white dark:bg-gray-800 text-gray-900 dark:text-white border border-gray-200 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-700" },
  { label: "5", action: "digit", value: "5", class: "bg-white dark:bg-gray-800 text-gray-900 dark:text-white border border-gray-200 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-700" },
  { label: "6", action: "digit", value: "6", class: "bg-white dark:bg-gray-800 text-gray-900 dark:text-white border border-gray-200 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-700" },
  { label: "−", action: "operator", value: "-", class: "bg-blue-100 dark:bg-blue-900/30 text-blue-600 dark:text-blue-400 hover:bg-blue-200 dark:hover:bg-blue-900/50" },

  { label: "1", action: "digit", value: "1", class: "bg-white dark:bg-gray-800 text-gray-900 dark:text-white border border-gray-200 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-700" },
  { label: "2", action: "digit", value: "2", class: "bg-white dark:bg-gray-800 text-gray-900 dark:text-white border border-gray-200 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-700" },
  { label: "3", action: "digit", value: "3", class: "bg-white dark:bg-gray-800 text-gray-900 dark:text-white border border-gray-200 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-700" },
  { label: "+", action: "operator", value: "+", class: "bg-blue-100 dark:bg-blue-900/30 text-blue-600 dark:text-blue-400 hover:bg-blue-200 dark:hover:bg-blue-900/50" },

  { label: "0", action: "digit", value: "0", class: "col-span-2 bg-white dark:bg-gray-800 text-gray-900 dark:text-white border border-gray-200 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-700" },
  { label: ".", action: "decimal", class: "bg-white dark:bg-gray-800 text-gray-900 dark:text-white border border-gray-200 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-700" },
  { label: "=", action: "equals", class: "bg-blue-600 hover:bg-blue-700 text-white" },
];

let operator = null;
let prevValue = null;

function handleButton(btn) {
  switch (btn.action) {
    case "digit":
      if (justCalculated.value) {
        currentInput.value = btn.value;
        justCalculated.value = false;
      } else {
        currentInput.value = currentInput.value === "0"
          ? btn.value
          : currentInput.value + btn.value;
      }
      break;

    case "decimal":
      if (!currentInput.value.includes(".")) {
        currentInput.value += ".";
      }
      justCalculated.value = false;
      break;

    case "operator":
      prevValue = parseFloat(currentInput.value);
      operator = btn.value;
      expression.value = `${prevValue.toLocaleString("id-ID")} ${btn.label}`;
      currentInput.value = "0";
      justCalculated.value = false;
      break;

    case "equals":
      if (operator && prevValue !== null) {
        const curr = parseFloat(currentInput.value);
        const opLabel = { "+": "+", "-": "−", "*": "×", "/": "÷" }[operator];
        expression.value = `${prevValue.toLocaleString("id-ID")} ${opLabel} ${curr.toLocaleString("id-ID")} =`;
        let result = 0;
        if (operator === "+") result = prevValue + curr;
        if (operator === "-") result = prevValue - curr;
        if (operator === "*") result = prevValue * curr;
        if (operator === "/") result = curr !== 0 ? prevValue / curr : 0;
        currentInput.value = String(Math.round(result * 100) / 100);
        operator = null;
        prevValue = null;
        justCalculated.value = true;
      }
      break;

    case "clear":
      currentInput.value = "0";
      expression.value = "";
      operator = null;
      prevValue = null;
      justCalculated.value = false;
      break;

    case "negate":
      currentInput.value = String(parseFloat(currentInput.value) * -1);
      break;

    case "percent":
      currentInput.value = String(parseFloat(currentInput.value) / 100);
      break;
  }
}

function sendToBayar() {
  const val = parseFloat(currentInput.value);
  if (!isNaN(val) && val > 0) {
    emit("send-to-bayar", Math.round(val));
  }
}
</script>

<style scoped>
.calc-slide-enter-active,
.calc-slide-leave-active {
  transition: all 0.2s ease;
}
.calc-slide-enter-from,
.calc-slide-leave-to {
  opacity: 0;
  transform: translateY(8px) scale(0.97);
}
</style>