import { ref } from "vue";

/**
 * useToast — Sistem notifikasi global.
 *
 * Cara pakai:
 *   import { useToast } from "@/composables/useToast";
 *   const toast = useToast();
 *   toast.success("Produk berhasil disimpan!");
 *   toast.error("Gagal menghapus data");
 *   toast.info("Data sedang dimuat...");
 *   toast.warning("Stok hampir habis");
 */

/** @type {import("vue").Ref<Array<{id:number,type:string,message:string}>>} */
const toasts = ref([]);
let nextId = 1;

const DURATION = 3500; // ms

function addToast(type, message) {
  const id = nextId++;
  toasts.value.push({ id, type, message });
  setTimeout(() => removeToast(id), DURATION);
}

function removeToast(id) {
  const idx = toasts.value.findIndex((t) => t.id === id);
  if (idx !== -1) toasts.value.splice(idx, 1);
}

export function useToast() {
  return {
    toasts,
    removeToast,
    success: (msg) => addToast("success", msg),
    error: (msg) => addToast("error", msg),
    info: (msg) => addToast("info", msg),
    warning: (msg) => addToast("warning", msg),
  };
}
