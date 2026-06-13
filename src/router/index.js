import { createRouter, createWebHistory } from "vue-router";

const routes = [
  {
    path: "/",
    name: "dashboard",
    component: () => import("../views/DashboardView.vue"),
    meta: { title: "Dashboard" },
  },
  {
    path: "/produk",
    name: "produk",
    component: () => import("../views/ProductView.vue"),
    meta: { title: "Produk & Stok" },
  },
  // Modul lain (Kasir, Percetakan, Jasa, Supplier, Laporan, Pengaturan)
  // akan ditambahkan di fase-fase berikutnya.
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
