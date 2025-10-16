import { createRouter, createWebHistory } from "vue-router";
import LoginPage from "../pages/LoginPage.vue";
import RegisterPage from "../pages/RegisterPage.vue";
import NotesPage from "../pages/NotesPage.vue";

const routes = [
  { path: "/", redirect: "/notes" },
  { path: "/login", component: LoginPage },
  { path: "/register", component: RegisterPage },
  { path: "/notes", component: NotesPage },
];

export const router = createRouter({
  history: createWebHistory(),
  routes,
});

router.beforeEach((to, from, next) => {
  const token = localStorage.getItem("token");
  if (to.path === "/notes" && !token) {
    next("/login");
  } else {
    next();
  }
});