<template>
  <div class="card p-4">
    <div class="flex items-center justify-between mb-3">
      <h3 class="font-semibold">Tren Pendapatan</h3>
      <div class="flex gap-1">
        <button
          class="text-xs px-2 py-1 rounded transition-colors"
          :class="chartType === 'line' ? 'bg-blue-600 text-white' : 'bg-gray-100 dark:bg-gray-700 text-gray-500'"
          @click="chartType = 'line'"
        >
          Garis
        </button>
        <button
          class="text-xs px-2 py-1 rounded transition-colors"
          :class="chartType === 'bar' ? 'bg-blue-600 text-white' : 'bg-gray-100 dark:bg-gray-700 text-gray-500'"
          @click="chartType = 'bar'"
        >
          Batang
        </button>
      </div>
    </div>

    <div class="relative h-64">
      <canvas ref="canvasRef"></canvas>
    </div>

    <p v-if="points.length === 0" class="text-center text-gray-400 text-sm mt-4">
      Belum ada data transaksi pada rentang ini
    </p>
  </div>
</template>

<script setup>
import { ref, watch, onMounted, onBeforeUnmount } from "vue";

const props = defineProps({
  points: { type: Array, default: () => [] }, // [{ date, total, count }]
});

const canvasRef = ref(null);
const chartType = ref("line");
let chartInstance = null;
let ChartJS = null;

function formatDateLabel(dateStr) {
  const d = new Date(`${dateStr}T00:00:00`);
  return d.toLocaleDateString("id-ID", { day: "numeric", month: "short" });
}

async function renderChart() {
  if (!ChartJS) {
    const mod = await import("chart.js/auto");
    ChartJS = mod.default;
  }

  if (chartInstance) {
    chartInstance.destroy();
    chartInstance = null;
  }

  if (!canvasRef.value) return;

  const labels = props.points.map((p) => formatDateLabel(p.date));
  const data = props.points.map((p) => p.total);

  chartInstance = new ChartJS(canvasRef.value, {
    type: chartType.value,
    data: {
      labels,
      datasets: [
        {
          label: "Omset",
          data,
          borderColor: "#2563eb",
          backgroundColor: chartType.value === "bar" ? "#3b82f680" : "#2563eb20",
          tension: 0.3,
          fill: chartType.value === "line",
        },
      ],
    },
    options: {
      responsive: true,
      maintainAspectRatio: false,
      plugins: {
        legend: { display: false },
        tooltip: {
          callbacks: {
            label: (ctx) => `Rp ${ctx.parsed.y.toLocaleString("id-ID")}`,
          },
        },
      },
      scales: {
        y: {
          ticks: {
            callback: (value) => `${(value / 1000).toLocaleString("id-ID")}rb`,
          },
        },
      },
    },
  });
}

watch(() => props.points, renderChart, { deep: true });
watch(chartType, renderChart);

onMounted(renderChart);

onBeforeUnmount(() => {
  chartInstance?.destroy();
});
</script>