import { onMounted, onUnmounted } from "vue";

/**
 * Composable shortcut keyboard untuk halaman Kasir/POS.
 * Hanya aktif ketika tidak ada input/textarea yang sedang fokus
 * (kecuali shortcut yang memang untuk field tertentu).
 *
 * @param {Object} handlers - { onF1, onF2, onF3, onF12, onEscape }
 */
export function useKeyboardShortcut(handlers = {}) {
  function handleKeydown(e) {
    // Jangan intercept kalau user sedang ketik di input biasa
    // (kecuali field yang punya data-barcode-input)
    const tag = document.activeElement?.tagName;
    const isTyping =
      (tag === "INPUT" || tag === "TEXTAREA" || tag === "SELECT") &&
      !document.activeElement?.dataset?.shortcutTarget;

    switch (e.key) {
      case "F1":
        e.preventDefault();
        handlers.onF1?.();
        break;

      case "F2":
        e.preventDefault();
        handlers.onF2?.();
        break;

      case "F3":
        e.preventDefault();
        handlers.onF3?.();
        break;

      case "F12":
        e.preventDefault();
        handlers.onF12?.();
        break;

      case "Escape":
        handlers.onEscape?.();
        break;

      case "Enter":
        // Enter di field bayar → trigger checkout
        if (document.activeElement?.dataset?.shortcutTarget === "bayar") {
          e.preventDefault();
          handlers.onEnterBayar?.();
        }
        break;

      default:
        break;
    }
  }

  onMounted(() => window.addEventListener("keydown", handleKeydown));
  onUnmounted(() => window.removeEventListener("keydown", handleKeydown));
}
