// usePrint.js — helper print kompatibel Tauri
// Pakai blob URL karena iframe.contentDocument bisa null di Tauri sandbox

export function printHtml(htmlContent) {
  return new Promise((resolve, reject) => {
    try {
      // Buat blob URL dari HTML string
      const blob = new Blob([htmlContent], { type: "text/html;charset=utf-8" });
      const url = URL.createObjectURL(blob);

      // Hapus iframe lama kalau ada
      const old = document.getElementById("__print_frame__");
      if (old) old.remove();

      const iframe = document.createElement("iframe");
      iframe.id = "__print_frame__";
      iframe.style.cssText =
        "position:fixed;top:-9999px;left:-9999px;width:1px;height:1px;border:none;opacity:0;";

      iframe.onload = () => {
        try {
          iframe.contentWindow.focus();
          iframe.contentWindow.print();
          setTimeout(() => {
            iframe.remove();
            URL.revokeObjectURL(url);
            resolve();
          }, 500);
        } catch (err) {
          iframe.remove();
          URL.revokeObjectURL(url);
          reject(err);
        }
      };

      iframe.onerror = (err) => {
        iframe.remove();
        URL.revokeObjectURL(url);
        reject(err);
      };

      iframe.src = url;
      document.body.appendChild(iframe);
    } catch (err) {
      reject(err);
    }
  });
}
