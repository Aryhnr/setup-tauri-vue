import { createRouter, createWebHistory } from "vue-router";

const routes = [
  {
    path: "/",
    name: "dashboard",
    component: () => import("../views/DashboardView.vue"),
    meta: { title: "Dashboard" },
  },
  {
    path: "/kasir",
    name: "kasir",
    component: () => import("../views/PosView.vue"),
    meta: { title: "Kasir / POS" },
  },
  {
    path: "/produk",
    name: "produk",
    component: () => import("../views/ProductView.vue"),
    meta: { title: "Produk" },
  },
  {
    path: "/stok",
    name: "stok",
    component: () => import("../views/StockView.vue"),
    meta: { title: "Manajemen Stok" },
  },
  {
    path: "/percetakan",
    name: "percetakan",
    component: () => import("../views/PrintView.vue"),
    meta: { title: "Percetakan" },
  },
  {
    path: "/jasa",
    name: "jasa",
    component: () => import("../views/ServiceView.vue"),
    meta: { title: "Jasa & Layanan" },
  },
  {
    path: "/supplier",
    name: "supplier",
    component: () => import("../views/SupplierView.vue"),
    meta: { title: "Supplier" },
  },
  {
    path: "/transaksi",
    name: "transaksi",
    component: () => import("../views/TransactionView.vue"),
    meta: { title: "Riwayat Transaksi" },
  },
  {
    path: "/laporan",
    name: "laporan",
    component: () => import("../views/ReportView.vue"),
    meta: { title: "Laporan" },
  },
  {
    path: "/pengaturan",
    name: "pengaturan",
    component: () => import("../views/SettingView.vue"),
    meta: { title: "Pengaturan" },
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
