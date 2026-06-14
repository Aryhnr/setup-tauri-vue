import { save } from "@tauri-apps/plugin-dialog";
import { writeFile } from "@tauri-apps/plugin-fs";

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
        mapped[col.label] = row[col.key] ?? "-";
      }
      return mapped;
    });
  }

  const worksheet = XLSX.utils.json_to_sheet(data);
  const workbook = XLSX.utils.book_new();
  XLSX.utils.book_append_sheet(workbook, worksheet, sheetName);

  // Buat buffer dulu, jangan langsung writeFile browser
  const buf = XLSX.write(workbook, { bookType: "xlsx", type: "array" });

  // Buka dialog "Simpan Sebagai" via Tauri
  const savePath = await save({
    defaultPath: `${filename}.xlsx`,
    filters: [{ name: "Excel", extensions: ["xlsx"] }],
  });

  if (!savePath) return; // user klik Cancel

  await writeFile(savePath, new Uint8Array(buf));
}

export async function exportToPdf({
  filename,
  title,
  subtitle,
  columns,
  rows,
  summary,
}) {
  const jsPDFModule = await import("jspdf");
  const jsPDF = jsPDFModule.default ?? jsPDFModule.jsPDF;
  await import("jspdf-autotable");

  const doc = new jsPDF();

  doc.setFontSize(16);
  doc.setTextColor(30, 30, 30);
  doc.text(title, 14, 18);

  let cursorY = 26;

  if (subtitle) {
    doc.setFontSize(9);
    doc.setTextColor(120, 120, 120);
    doc.text(subtitle, 14, cursorY);
    doc.setTextColor(30, 30, 30);
    cursorY += 8;
  }

  if (summary && summary.length > 0) {
    doc.setFontSize(10);
    for (const item of summary) {
      doc.text(`${item.label}: ${item.value}`, 14, cursorY);
      cursorY += 6;
    }
    cursorY += 2;
  }

  doc.autoTable({
    startY: cursorY,
    head: [columns.map((c) => c.label)],
    body: rows.map((row) => columns.map((c) => formatCell(row[c.key]))),
    styles: { fontSize: 9, cellPadding: 3 },
    headStyles: { fillColor: [37, 99, 235], textColor: 255, fontStyle: "bold" },
    alternateRowStyles: { fillColor: [245, 247, 250] },
    margin: { left: 14, right: 14 },
  });

  // Output ke arraybuffer, bukan doc.save() langsung
  const pdfBytes = doc.output("arraybuffer");

  // Buka dialog "Simpan Sebagai" via Tauri
  const savePath = await save({
    defaultPath: `${filename}.pdf`,
    filters: [{ name: "PDF", extensions: ["pdf"] }],
  });

  if (!savePath) return; // user klik Cancel

  await writeFile(savePath, new Uint8Array(pdfBytes));
}

function formatCell(value) {
  if (value === null || value === undefined) return "-";
  if (typeof value === "number") return value.toLocaleString("id-ID");
  return String(value);
}
