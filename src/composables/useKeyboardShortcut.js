import { onMounted, onUnmounted } from "vue";

export function useKeyboardShortcut(handlers = {}) {
  function handleKeydown(e) {
    const active = document.activeElement;
    const tag = active?.tagName;
    const isTypingField =
      (tag === "INPUT" || tag === "TEXTAREA" || tag === "SELECT") &&
      !active?.dataset?.shortcutTarget;

    switch (e.key) {
      case "F1":
        e.preventDefault();
        handlers.onF1?.();
        return;

      case "F2":
        e.preventDefault();
        handlers.onF2?.();
        return;

      case "F3":
        e.preventDefault();
        handlers.onF3?.();
        return;

      case "F5":
        e.preventDefault();
        handlers.onF5?.();
        return;

      case "F12":
        e.preventDefault();
        handlers.onF12?.();
        return;

      case "Escape":
        e.preventDefault();
        handlers.onEscape?.();
        return;

      case "Enter":
        if (active?.dataset?.shortcutTarget === "bayar") {
          return;
        }
        if (!isTypingField && handlers.onEnter) {
          e.preventDefault();
          handlers.onEnter?.();
        }
        return;

      default:
        break;
    }
  }

  onMounted(() => window.addEventListener("keydown", handleKeydown));
  onUnmounted(() => window.removeEventListener("keydown", handleKeydown));
}
