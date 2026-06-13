/**
 * Composable untuk export data laporan ke PDF dan Excel (.xlsx).
 *
 * Implementasi sisi frontend menggunakan `jspdf` + `jspdf-autotable`
 * (PDF) dan `xlsx` / SheetJS (Excel). Kedua library di-load secara
 * dynamic import agar tidak membengkakkan bundle awal aplikasi.
 *
 * Install dependency (sekali saja):
 *   pnpm add jspdf jspdf-autotable xlsx
 */

/**
 * Export data tabel ke file Excel (.xlsx).
 * @param {Object} options
 * @param {string} options.filename - nama file tanpa ekstensi
 * @param {string} options.sheetName - nama sheet
 * @param {Array<Object>} options.rows - data baris (array of object)
 * @param {Array<{key: string, label: string}>} [options.columns] - jika diisi,
 *        kolom akan diurutkan & diberi header sesuai `label`. Jika tidak,
 *        seluruh key dari object pertama dipakai sebagai header.
 */
export async function exportToExcel({
  filename,
  sheetName = "Laporan",
  rows,
  columns,
}) {
  const XLSX = await import("xlsx");

  let data = rows;

  if (columns && columns.length > 0) {
    data = rows.map((row) => {
      const mapped = {};
      for (const col of columns) {
        mapped[col.label] = row[col.key];
      }
      return mapped;
    });
  }

  const worksheet = XLSX.utils.json_to_sheet(data);
  const workbook = XLSX.utils.book_new();
  XLSX.utils.book_append_sheet(workbook, worksheet, sheetName);

  XLSX.writeFile(workbook, `${filename}.xlsx`);
}

/**
 * Export data tabel ke file PDF dengan judul, sub-judul (opsional),
 * dan tabel otomatis (pakai jspdf-autotable).
 * @param {Object} options
 * @param {string} options.filename - nama file tanpa ekstensi
 * @param {string} options.title - judul laporan
 * @param {string} [options.subtitle] - sub-judul (misal rentang tanggal)
 * @param {Array<{key: string, label: string}>} options.columns
 * @param {Array<Object>} options.rows
 * @param {Array<{label: string, value: string}>} [options.summary] - ringkasan
 *        di atas tabel, misal "Total Omset: Rp 1.000.000"
 */
export async function exportToPdf({
  filename,
  title,
  subtitle,
  columns,
  rows,
  summary,
}) {
  const { default: jsPDF } = await import("jspdf");
  const { default: autoTable } = await import("jspdf-autotable");

  const doc = new jsPDF();

  doc.setFontSize(16);
  doc.text(title, 14, 18);

  let cursorY = 26;

  if (subtitle) {
    doc.setFontSize(10);
    doc.setTextColor(120);
    doc.text(subtitle, 14, cursorY);
    doc.setTextColor(0);
    cursorY += 8;
  }

  if (summary && summary.length > 0) {
    doc.setFontSize(11);
    for (const item of summary) {
      doc.text(`${item.label}: ${item.value}`, 14, cursorY);
      cursorY += 6;
    }
    cursorY += 2;
  }

  autoTable(doc, {
    startY: cursorY,
    head: [columns.map((c) => c.label)],
    body: rows.map((row) => columns.map((c) => formatCell(row[c.key]))),
    styles: { fontSize: 9 },
    headStyles: { fillColor: [37, 99, 235] },
  });

  doc.save(`${filename}.pdf`);
}

function formatCell(value) {
  if (value === null || value === undefined) return "-";
  if (typeof value === "number") {
    return value.toLocaleString("id-ID");
  }
  return String(value);
}
