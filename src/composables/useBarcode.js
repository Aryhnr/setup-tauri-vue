import { onMounted, onUnmounted } from "vue";

/**
 * Composable untuk mendeteksi input dari barcode scanner USB (plug & play,
 * bekerja sebagai keyboard HID). Scanner mengetik karakter barcode secara
 * sangat cepat lalu diakhiri dengan Enter.
 *
 * Strategi: kumpulkan karakter yang diketik dalam jeda waktu singkat
 * (< `maxIntervalMs` antar karakter). Jika jeda lebih lama, buffer dianggap
 * input manual biasa dan di-reset. Saat Enter ditekan dan buffer cukup
 * panjang, anggap sebagai hasil scan barcode dan panggil `onScan`.
 *
 * @param {(barcode: string) => void} onScan - dipanggil dengan hasil scan
 * @param {Object} options
 * @param {number} options.minLength - panjang minimum barcode yang valid (default 3)
 * @param {number} options.maxIntervalMs - jeda maksimum antar karakter dalam ms (default 50)
 */
export function useBarcode(onScan, options = {}) {
  const minLength = options.minLength ?? 3;
  const maxIntervalMs = options.maxIntervalMs ?? 50;

  let buffer = "";
  let lastTime = 0;

  function handleKeydown(e) {
    // Jangan tangkap input saat user sedang mengetik di field teks biasa,
    // kecuali field tersebut secara eksplisit menandai diri sebagai
    // penerima scan (data-barcode-input).
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
        onScan(buffer);
        e.preventDefault();
      }
      buffer = "";
      return;
    }

    // Reset buffer jika jeda terlalu lama (kemungkinan ketikan manual)
    if (elapsed > maxIntervalMs) {
      buffer = "";
    }

    // Hanya tangkap karakter alfanumerik (barcode umumnya angka/huruf)
    if (e.key.length === 1 && /[a-zA-Z0-9]/.test(e.key)) {
      // Jika sedang fokus di input teks biasa, jangan ganggu (biarkan
      // ketikan masuk ke input seperti biasa), tapi tetap track buffer
      // untuk deteksi pola scan cepat.
      buffer += e.key;
      if (isTextInput) {
        // tidak preventDefault, biarkan input normal jalan
      }
    }
  }

  onMounted(() => {
    window.addEventListener("keydown", handleKeydown);
  });

  onUnmounted(() => {
    window.removeEventListener("keydown", handleKeydown);
  });
}
