<template>
  <div class="space-y-6 max-w-2xl">

    <div class="card p-6 space-y-4 border border-gray-200 dark:border-gray-800 rounded-lg">
      <h2 class="text-base font-semibold text-gray-800 dark:text-white flex items-center gap-2">
        <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5 text-gray-500 dark:text-gray-400" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m2 7 4.41-4.41A2 2 0 0 1 7.83 2h8.34a2 2 0 0 1 1.42.59L22 7"/><path d="M4 12v8a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-8"/><path d="M15 22v-4a2 2 0 0 0-2-2h-2a2 2 0 0 0-2 2v4"/><path d="M2 7h20"/><path d="M22 12H2"/></svg>
        Informasi Toko
      </h2>
      <form class="space-y-3" @submit.prevent="saveInfo">
        <BaseInput v-model="form.storeName" label="Nama Toko" placeholder="Toko Madura" required />
        <BaseInput v-model="form.storePhone" label="Nomor Telepon" placeholder="08xxxxxxxxxx" />
        <div>
          <label class="label">Alamat</label>
          <textarea
            v-model="form.storeAddress"
            class="input resize-none"
            rows="2"
            placeholder="Jl. Raya No. 1, Surabaya"
          ></textarea>
        </div>
        <div class="flex justify-end">
          <BaseButton type="submit">Simpan Informasi</BaseButton>
        </div>
      </form>
      <p v-if="savedInfo" class="text-sm text-green-600 dark:text-green-400 text-right">✓ Tersimpan!</p>
    </div>

    <div class="card p-6 space-y-4 border border-gray-200 dark:border-gray-800 rounded-lg">
      <h2 class="text-base font-semibold text-gray-800 dark:text-white flex items-center gap-2">
        <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5 text-gray-500 dark:text-gray-400" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 22c5.523 0 10-2.239 10-5s-4.477-5-10-5-10 2.239-10 5 4.477 5 10 5z"/><path d="M22 5c0 2.761-4.477 5-10 5S2 7.761 2 5s4.477-5 10-5 10 2.239 10 5z"/><path d="M2 5v6c0 2.761 4.477 5 10 5s10-2.239 10-5V5"/><path d="M2 11v6c0 2.761 4.477 5 10 5s10-2.239 10-5v-6"/></svg>
        Backup & Restore Database
      </h2>

      <div class="bg-gray-50 dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-md p-3 text-xs text-gray-500 dark:text-gray-400 font-mono break-all flex items-center gap-2">
        <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4 text-gray-400 shrink-0" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"/><path d="M2 10h20"/></svg>
        <span>Lokasi database: {{ appDataDir || "Memuat..." }}</span>
      </div>

      <div class="space-y-2">
        <p class="text-sm font-medium text-gray-700 dark:text-gray-300">Backup Manual</p>
        <p class="text-xs text-gray-400">Klik kolom di bawah untuk memilih folder tujuan secara visual, lalu klik Backup.</p>
        <div class="flex gap-2">
          <input
            :value="backupDir"
            type="text"
            readonly
            @click="selectBackupDir"
            class="input flex-1 text-sm cursor-pointer bg-gray-50/50 dark:bg-gray-950/50 hover:border-gray-300 dark:hover:border-gray-700 transition-colors truncate"
            placeholder="Klik untuk memilih folder tujuan backup..."
          />
          <BaseButton @click="doBackup" :disabled="backupLoading || !backupDir">
            {{ backupLoading ? "Membackup..." : "Backup" }}
          </BaseButton>
        </div>
        <p v-if="backupSuccess" class="text-sm text-green-600 dark:text-green-400">
          ✓ Backup berhasil: <span class="font-mono text-xs">{{ backupResult }}</span>
        </p>
        <p v-if="backupError" class="text-sm text-red-500">{{ backupError }}</p>
      </div>

      <div class="border-t border-gray-200 dark:border-gray-700 pt-4 space-y-2">
        <p class="text-sm font-medium text-gray-700 dark:text-gray-300">Restore dari Backup</p>
        <div class="bg-yellow-50 dark:bg-yellow-950/30 border border-yellow-200 dark:border-yellow-900/50 rounded-md p-3 text-xs text-yellow-700 dark:text-yellow-400 flex gap-2">
          <svg xmlns="http://www.w3.org/2000/svg" class="w-4 h-4 shrink-0 mt-0.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3Z"/><line x1="12" y1="9" x2="12" y2="13"/><line x1="12" y1="17" x2="12.01" y2="17"/></svg>
          <p>Restore akan mengganti seluruh data dengan isi file backup. Pastikan kamu yakin sebelum melanjutkan.</p>
        </div>
        <div class="flex gap-2">
          <input
            :value="restoreFile"
            type="text"
            readonly
            @click="selectRestoreFile"
            class="input flex-1 text-sm cursor-pointer bg-gray-50/50 dark:bg-gray-950/50 hover:border-gray-300 dark:hover:border-gray-700 transition-colors truncate"
            placeholder="Klik untuk memilih file cadangan (.db)..."
          />
          <BaseButton variant="danger" @click="confirmRestore" :disabled="restoreLoading || !restoreFile">
            {{ restoreLoading ? "Merestore..." : "Restore" }}
          </BaseButton>
        </div>
        <p v-if="restoreSuccess" class="text-sm text-green-600 dark:text-green-400">
          ✓ Restore berhasil! Tutup dan buka ulang aplikasi untuk menerapkan perubahan.
        </p>
        <p v-if="restoreError" class="text-sm text-red-500">{{ restoreError }}</p>
      </div>
    </div>

    <div class="card p-6 space-y-2 border border-gray-200 dark:border-gray-800 rounded-lg">
      <h2 class="text-base font-semibold text-gray-800 dark:text-white flex items-center gap-2">
        <svg xmlns="http://www.w3.org/2000/svg" class="w-5 h-5 text-gray-500 dark:text-gray-400" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="M12 16v-4"/><path d="M12 8h.01"/></svg>
        Tentang Aplikasi
      </h2>
      <div class="text-sm text-gray-500 dark:text-gray-400 space-y-1 pt-1">
        <p><span class="font-medium text-gray-700 dark:text-gray-300">Aplikasi:</span> Sistem Manajemen Toko Madura</p>
        <p><span class="font-medium text-gray-700 dark:text-gray-300">Versi:</span> 1.0.0</p>
        <p><span class="font-medium text-gray-700 dark:text-gray-300">Stack:</span> Tauri v2 + Vue.js v3 + SQLite</p>
      </div>
    </div>

    <BaseModal v-model="showRestoreConfirm" title="Konfirmasi Restore">
      <p class="text-sm text-gray-600 dark:text-gray-300 mb-2">
        Kamu yakin ingin merestore database dari file:
      </p>
      <p class="text-xs font-mono bg-gray-100 dark:bg-gray-800 p-2 rounded mb-4 break-all border border-gray-200 dark:border-gray-700">{{ restoreFile }}</p>
      <p class="text-sm font-medium text-red-600 dark:text-red-400 mb-4">
        Seluruh data yang ada sekarang akan diganti. Tindakan ini tidak bisa dibatalkan.
      </p>
      <div class="flex justify-end gap-2">
        <BaseButton variant="secondary" @click="showRestoreConfirm = false">Batal</BaseButton>
        <BaseButton variant="danger" @click="doRestore">Ya, Restore Sekarang</BaseButton>
      </div>
    </BaseModal>
  </div>
</template>

<script setup>
import { ref, reactive, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { useSettingStore } from "../stores/settingStore";
import BaseButton from "../components/ui/BaseButton.vue";
import BaseInput from "../components/ui/BaseInput.vue";
import BaseModal from "../components/ui/BaseModal.vue";
import { useToast } from "../composables/useToast";

const settingStore = useSettingStore();
const toast = useToast();

const form = reactive({
  storeName: settingStore.storeName,
  storeAddress: settingStore.storeAddress,
  storePhone: settingStore.storePhone,
});

const savedInfo = ref(false);
const appDataDir = ref("");
const backupDir = ref(settingStore.backupDir || "");
const restoreFile = ref("");

const backupLoading = ref(false);
const backupSuccess = ref(false);
const backupResult = ref("");
const backupError = ref("");

const restoreLoading = ref(false);
const restoreSuccess = ref(false);
const restoreError = ref("");
const showRestoreConfirm = ref(false);

function saveInfo() {
  settingStore.save({
    storeName: form.storeName,
    storeAddress: form.storeAddress,
    storePhone: form.storePhone,
  });
  savedInfo.value = true;
  setTimeout(() => (savedInfo.value = false), 2500);
  toast.success("Informasi toko berhasil disimpan");
}

async function selectBackupDir() {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      defaultPath: backupDir.value || undefined,
    });
    if (selected) {
      backupDir.value = selected;
      backupError.value = "";
    }
  } catch (err) {
    backupError.value = "Gagal membuka jendela pemilihan folder";
  }
}

async function selectRestoreFile() {
  try {
    const selected = await open({
      directory: false,
      multiple: false,
      filters: [{ name: "SQLite Database", extensions: ["db", "sqlite"] }],
    });
    if (selected) {
      restoreFile.value = selected;
      restoreError.value = "";
    }
  } catch (err) {
    restoreError.value = "Gagal membuka jendela pemilihan file";
  }
}

async function doBackup() {
  if (!backupDir.value) {
    backupError.value = "Silakan pilih folder tujuan backup terlebih dahulu";
    return;
  }
  backupLoading.value = true;
  backupSuccess.value = false;
  backupError.value = "";
  try {
    const resultPath = await invoke("backup_database", { destDir: backupDir.value });
    backupResult.value = resultPath;
    backupSuccess.value = true;
    settingStore.save({ backupDir: backupDir.value });
    toast.success("Backup database berhasil!");
  } catch (err) {
    backupError.value = err?.toString() ?? "Backup gagal";
    toast.error("Backup gagal: " + (err?.toString() ?? ""));
  } finally {
    backupLoading.value = false;
  }
}

function confirmRestore() {
  if (!restoreFile.value) {
    restoreError.value = "Silakan pilih file backup terlebih dahulu";
    return;
  }
  restoreError.value = "";
  showRestoreConfirm.value = true;
}

async function doRestore() {
  showRestoreConfirm.value = false;
  restoreLoading.value = true;
  restoreSuccess.value = false;
  restoreError.value = "";
  try {
    await invoke("restore_database", { backupFile: restoreFile.value });
    restoreSuccess.value = true;
    toast.success("Restore database berhasil! Restart aplikasi untuk memuat data terbaru.");
  } catch (err) {
    restoreError.value = err?.toString() ?? "Restore gagal";
  } finally {
    restoreLoading.value = false;
  }
}

onMounted(async () => {
  try {
    appDataDir.value = await invoke("get_app_data_dir");
  } catch {
    appDataDir.value = "(tidak tersedia)";
  }
});
</script>