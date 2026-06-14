import { defineStore } from "pinia";

const STORAGE_KEY = "toko_settings";

const defaultSettings = {
  storeName: "Toko Madura",
  storeAddress: "",
  storePhone: "",
  backupDir: "",
  autoBackup: false,
};

export const useSettingStore = defineStore("setting", {
  state: () => ({
    ...loadSettings(),
  }),

  actions: {
    save(payload) {
      this.storeName = payload.storeName ?? this.storeName;
      this.storeAddress = payload.storeAddress ?? this.storeAddress;
      this.storePhone = payload.storePhone ?? this.storePhone;
      this.backupDir = payload.backupDir ?? this.backupDir;
      this.autoBackup = payload.autoBackup ?? this.autoBackup;
      persistSettings(this.$state);
    },
  },
});

function loadSettings() {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (raw) return { ...defaultSettings, ...JSON.parse(raw) };
  } catch {
    // ignore
  }
  return { ...defaultSettings };
}

function persistSettings(state) {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(state));
  } catch {
    // ignore
  }
}
