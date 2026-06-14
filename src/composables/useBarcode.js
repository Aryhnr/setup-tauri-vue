import { onMounted, onUnmounted, ref } from "vue";

/**
 * State global — shared supaya notifikasi "scanner siap" bisa
 * ditampilkan dari mana saja (App.vue, PosView, dll).
 */
export const scannerReady = ref(false);
export const scannerReadyMessage = ref("");

let readyTimer = null;

function showScannerReady() {
  scannerReady.value = true;
  scannerReadyMessage.value =
    "🔫 Barcode scanner terdeteksi dan siap digunakan!";
  clearTimeout(readyTimer);
  readyTimer = setTimeout(() => {
    scannerReady.value = false;
  }, 4000);
}

/**
 * Composable untuk mendeteksi input dari barcode scanner USB (plug & play,
 * bekerja sebagai keyboard HID). Scanner mengetik karakter barcode secara
 * sangat cepat lalu diakhiri dengan Enter.
 *
 * @param {(barcode: string) => void} onScan - dipanggil dengan hasil scan
 * @param {Object} options
 * @param {number} options.minLength - panjang minimum barcode valid (default 3)
 * @param {number} options.maxIntervalMs - jeda maks antar karakter dalam ms (default 50)
 */
export function useBarcode(onScan, options = {}) {
  const minLength = options.minLength ?? 3;
  const maxIntervalMs = options.maxIntervalMs ?? 50;

  let buffer = "";
  let lastTime = 0;
  let scanDetectedOnce = false;

  function handleKeydown(e) {
    const target = e.target;
    const isTextInput =
      target &&
      (target.tagName === "INPUT" || target.tagName === "TEXTAREA") &&
      !target.dataset.barcodeInput;

    const now = Date.now();
    const elapsed = now - lastTime;
    lastTime = now;

    if (e.key === "Enter") {
      if (buffer.length >= minLength && elapsed < maxIntervalMs) {
        if (!scanDetectedOnce) {
          scanDetectedOnce = true;
          showScannerReady();
        }
        onScan(buffer);
        e.preventDefault();
      }
      buffer = "";
      return;
    }

    if (elapsed > maxIntervalMs) {
      buffer = "";
    }

    if (e.key.length === 1 && /[a-zA-Z0-9]/.test(e.key)) {
      buffer += e.key;
    }
  }

  onMounted(() => {
    window.addEventListener("keydown", handleKeydown);
  });

  onUnmounted(() => {
    window.removeEventListener("keydown", handleKeydown);
  });
}
