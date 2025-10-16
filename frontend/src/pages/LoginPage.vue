<template>
  <div class="container mt-5" style="max-width: 400px;">
    <h2 class="mb-4">Вход</h2>
    <div class="mb-3">
      <label class="form-label">Email или логин</label>
      <input v-model="username" type="text" class="form-control" />
    </div>
    <div class="mb-3">
      <label class="form-label">Пароль</label>
      <input v-model="password" type="password" class="form-control" />
    </div>
    <button class="btn btn-primary w-100" @click="login">Войти</button>
    <div class="mt-3 text-center">
      <router-link to="/register">Зарегистрироваться</router-link>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import api from "../api";
import { useRouter } from "vue-router";

const username = ref("");
const password = ref("");
const router = useRouter();

const login = async () => {
  try {
    const resp = await api.post("/auth/login", {
      username: username.value,
      password: password.value,
    });
    const token = resp.data.token; // предполагаем, что бек возвращает { token: "…" }
    localStorage.setItem("token", token);
    await router.push("/notes");
  } catch (err) {
    alert("Ошибка логина");
    console.error(err);
  }
};
</script>