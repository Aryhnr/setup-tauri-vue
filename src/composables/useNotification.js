import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

// State global — shared antar komponen yang pakai composable ini
const lowStockItems = ref([]);
const showAlert = ref(false);
let audioCtx = null;

/**
 * Mainkan bunyi beep sederhana via Web Audio API.
 * Tidak butuh file audio eksternal.
 */
function playBeep() {
  try {
    if (!audioCtx) audioCtx = new AudioContext();
    const oscillator = audioCtx.createOscillator();
    const gainNode = audioCtx.createGain();
    oscillator.connect(gainNode);
    gainNode.connect(audioCtx.destination);
    oscillator.type = "sine";
    oscillator.frequency.value = 880; // A5
    gainNode.gain.setValueAtTime(0.4, audioCtx.currentTime);
    gainNode.gain.exponentialRampToValueAtTime(
      0.001,
      audioCtx.currentTime + 0.5,
    );
    oscillator.start(audioCtx.currentTime);
    oscillator.stop(audioCtx.currentTime + 0.5);
  } catch {
    // AudioContext tidak tersedia — abaikan
  }
}

export function useNotification() {
  /**
   * Cek stok menipis dari backend.
   * Jika ada produk yang stoknya <= min_stock, tampilkan popup + bunyi.
   * Dipanggil saat startup dan setelah setiap transaksi.
   */
  async function checkLowStock() {
    try {
      const items = await invoke("get_low_stock_products");
      lowStockItems.value = items;
      if (items.length > 0) {
        showAlert.value = true;
        playBeep();
      }
    } catch (err) {
      console.error("Gagal cek stok menipis:", err);
    }
  }

  function dismissAlert() {
    showAlert.value = false;
  }

  return {
    lowStockItems,
    showAlert,
    checkLowStock,
    dismissAlert,
  };
}
