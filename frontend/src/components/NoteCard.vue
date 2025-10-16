<template>
  <div class="card h-100">
    <div class="card-body">
      <h5 class="card-title">{{ note.title }}</h5>
      <p class="card-text">{{ note.text }}</p>
    </div>
    <div class="card-footer text-end">
      <button class="btn btn-sm btn-warning me-2" @click="$emit('edit', note)">
        Редактировать
      </button>
      <button class="btn btn-sm btn-danger" @click="$emit('delete', note.id)">
        Удалить
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { defineProps } from "vue";

interface Note {
  id: number;
  title: string;
  text: string;
}

const props = defineProps<{
  note: Note;
}>();
</script>