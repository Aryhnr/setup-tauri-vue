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
    meta: { title: "Produk & Stok" },
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
    path: "/laporan",
    name: "laporan",
    component: () => import("../views/ReportView.vue"),
    meta: { title: "Laporan" },
  },
  // Modul lain (Supplier, Pengaturan)
  // akan ditambahkan di fase-fase berikutnya.
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
