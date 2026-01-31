<script setup>
import { ref } from "vue"
import { invoke } from "@tauri-apps/api/core"

const title = ref("")
const description = ref("")
const result = ref(null)
const error = ref(null)

async function createTask() {
  error.value = null
  result.value = null

  try {
    const id = await invoke("create_task", {
      title: title.value,
      description: description.value || null
    })
    result.value = `Task created with ID ${id}`
    title.value = ""
    description.value = ""
  } catch (e) {
    error.value = e
  }
}
</script>

<template>
  <main style="padding: 2rem; max-width: 400px;">
    <h1>Create Task</h1>

    <input
      v-model="title"
      placeholder="Title"
      style="display: block; width: 100%; margin-bottom: 0.5rem;"
    />

    <textarea
      v-model="description"
      placeholder="Description"
      style="display: block; width: 100%; margin-bottom: 0.5rem;"
    />

    <button @click="createTask">Create</button>

    <p v-if="result" style="color: green;">{{ result }}</p>
    <p v-if="error" style="color: red;">{{ error }}</p>
  </main>
</template>
