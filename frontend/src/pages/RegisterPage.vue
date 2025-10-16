<template>
  <div class="container mt-5" style="max-width: 400px;">
    <h2 class="mb-4">Регистрация</h2>
    <div class="mb-3">
      <label class="form-label">Имя пользователя</label>
      <input v-model="username" type="text" class="form-control" />
    </div>
    <div class="mb-3">
      <label class="form-label">Пароль</label>
      <input v-model="password" type="password" class="form-control" />
    </div>
        <div class="mb-3">
      <label class="form-label">почта</label>
      <input v-model="email" type="password" class="form-control" />
    </div>
    <button class="btn btn-primary w-100" @click="register">Зарегистрироваться</button>
    <div class="mt-3 text-center">
      <router-link to="/login">Уже есть аккаунт?</router-link>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import api from "../api";
import { useRouter } from "vue-router";

const username = ref("");
const password = ref("");
const email = ref("");
const router = useRouter();

const register = async () => {
  try {
    await api.post("/register", {
      username: username.value,
      password: password.value,
      email: email.value

    });
    // После регистрации автоматически логиним или редиректим на логин
    await router.push("/auth/login");
  } catch (err) {
    alert("Ошибка регистрации");
    console.error(err);
  }
};
</script>