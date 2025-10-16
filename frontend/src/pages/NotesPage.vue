<template>
  <div class="container py-4">
    <h1 class="mb-4">Мои заметки</h1>

    <!-- Карточка формы создания/редактирования -->
    <div class="card mb-4">
      <div class="card-body">
        <h5 class="card-title">{{ editingId === null ? "Новая заметка" : "Редактировать" }}</h5>
        <div class="mb-3">
          <input
            v-model="form.title"
            type="text"
            class="form-control"
            placeholder="Заголовок"
          />
        </div>
        <div class="mb-3">
          <textarea
            v-model="form.text"
            class="form-control"
            rows="4"
            placeholder="Текст"
          ></textarea>
        </div>
        <button class="btn btn-primary" @click="saveNote">Сохранить</button>
        <button v-if="editingId !== null" class="btn btn-secondary ms-2" @click="cancelEdit">Отменить</button>
      </div>
    </div>

    <!-- Список карточек заметок -->
    <div class="row row-cols-1 row-cols-md-2 g-4">
      <div v-for="note in notes" :key="note.id" class="col">
        <NoteCard
          :note="note"
          @edit="startEdit"
          @delete="deleteNote"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import api from "../api";
import NoteCard from "../components/NoteCard.vue";

interface Note {
  id: number;
  title: string;
  text: string;
}

const notes = ref<Note[]>([]);
const form = ref<{ title: string; text: string }>({
  title: "",
  text: "",
});
const editingId = ref<number | null>(null);

const fetchNotes = async () => {
  const resp = await api.get<Note[]>("/notes");
  notes.value = resp.data;
};

const saveNote = async () => {
  if (editingId.value === null) {
    // создать
    await api.post("/notes/", {
      title: form.value.title,
      text: form.value.text,
    });
  } else {
    // редактировать
    await api.put(`/notes/${editingId.value}`, {
      title: form.value.title,
      text: form.value.text,
    });
  }
  form.value.title = "";
  form.value.text = "";
  editingId.value = null;
  await fetchNotes();
};

const deleteNote = async (title: string) => {
  await api.delete(`/notes/${title}`);
  await fetchNotes();
};

const startEdit = (note: Note) => {
  editingId.value = note.id;
  form.value.title = note.title;
  form.value.text = note.text;
};

const cancelEdit = () => {
  editingId.value = null;
  form.value.title = "";
  form.value.text = "";
};

onMounted(fetchNotes);
</script>