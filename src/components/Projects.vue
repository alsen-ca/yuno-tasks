<template>
  <div class="projects-container">
    <div class="projects-header">
      <h2>Projects</h2>
      <button class="add-button" @click="openModal">
        <img src="../assets/plus.svg" alt="Add Project" class="plus-icon" />
      </button>
    </div>


    <div v-if="isModalOpen" class="modal-overlay" @click.self="closeModal">
      <div class="modal-content">
          <h3>New Project</h3>

          <input
            v-model="title"
            placeholder="Title"
            class="modal-input"
          />

          <textarea
            v-model="description"
            placeholder="Description"
            class="modal-textarea"
          />

          <div class="modal-actions">
              <button @click="createProject">Create</button>
              <button @click="closeModal" class="cancel-button">Cancel</button>
          </div>

          <p v-if="result" class="success-message">{{ result }}</p>
          <p v-if="error" class="error-message">{{ error }}</p>
      </div>
    </div>


    <div class="projects-list">
      <div v-for="project in projects" :key="project.id" class="project-item">
        <!-- Sequence: Input or Text -->
        <div v-if="editProjectId === project.id" class="sequence-edit">
          <button
            @click="project.sequence--"
            :disabled="project.sequence <= 1"
            class="sequence-button"
          >
            -
          </button>
          <input
            v-model.number="project.sequence"
            type="number"
            min="1"
            class="edit-sequence"
          />
          <button
            @click="project.sequence++"
            class="sequence-button"
          >
            +
          </button>
        </div>
        <div v-else class="project-sequence">{{ project.sequence }}</div>

        <!-- Title: Input or Text-->
        <input
          v-if="editProjectId === project.id"
          v-model="project.title"
          class="edit-input"
        />
        <div v-else class="project-title">{{ project.title }}</div>

        <!-- Description: Textarea or Text -->
        <textarea
          v-if="editProjectId === project.id"
          v-model="project.description"
          class="edit-textarea"
        />
        <div v-else class="project-description">
          {{ project.description }}
        </div>

        <div class="project-actions">
          <template v-if="editProjectId === project.id">
            <button @click="saveEdit(project)" class="save-button">Save</button>
            <button @click="cancelEdit" class="cancel-button">Cancel</button>
          </template>
          <template v-else>
            <button @click="startEditing(project.id)" class="edit-button">Edit</button>
            <button @click="() => deleteProject(project.id)" class="delete-button">Delete</button>
          </template>
        </div>
      </div>
    </div>
  </div>
  <p v-if="error" class="error-message">{{ error }}</p>
  <p v-if="result" class="success-message">{{ result }}</p>
</template>

<script setup>
import { ref, onMounted, watch } from 'vue';
import { invoke } from "@tauri-apps/api/core"

const isModalOpen = ref(false);
const title = ref("");
const description = ref("");
const result = ref(null);
const error = ref(null);
const projects = ref([]);
const editProjectId = ref(null);

watch([error, result], ([newError, newResult], [oldError, oldResult]) => {
  if (newError !== oldError && newError !== null) {
    setTimeout(() => {
      error.value = null;
    }, 60000); // 1 minute
  }

  if (newResult !== oldResult && newResult !== null) {
    setTimeout(() => {
      result.value = null;
    }, 60000)
  }
}, { deep: true });

function openModal() {
  isModalOpen.value = true;
}

function closeModal() {
  isModalOpen.value = false;
  title.value = "";
  description.value = "";
  result.value = null;
  error.value = null;
}

async function createProject() {
  error.value = null
  result.value = null

  try {
    const id = await invoke("create_project", {
      title: title.value,
      description: description.value || null
    });
    result.value = `Project created with ID ${id}`
    fetchProjects();
    closeModal();
  } catch (e) {
    error.value = e
  }
}

onMounted(async () => {
  await fetchProjects();
});

async function fetchProjects() {
  try {
    projects.value = await invoke("get_projects");
  } catch (e) {
    error.value = `Failed to fetch projects: ${e}`;
  }
}

function startEditing(id) {
  editProjectId.value = id;
}

async function saveEdit(project) {
  try {
    await invoke("update_project_content", {
      id: project.id,
      title: project.title,
      description: project.description,
    });
    await invoke("update_project_sequence", {
      id: project.id,
      sequence: project.sequence
    });

    editProjectId.value = null;
    await fetchProjects();
  } catch (e) {
    error.value = `Failed to update project: ${e}`;
  }
}

async function cancelEdit() {
  editProjectId.value = null;
  await fetchProjects();
}

async function deleteProject(id) {
  try {
    await invoke("delete_project", { id });
    await fetchProjects();
  } catch (e) {
    error.value = `Failed to delete project: ${e}`;
    console.error("Deletion error:", e);
  }
}
</script>

<style scoped>
.projects-container {
  padding: 1rem;
}

.projects-header {
  display: flex;
  justify-content: center;
  align-items: center;
  position: relative;
  margin-bottom: 1rem;
}

.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
}

.modal-content {
  background-color: var(--bg-darker);
  padding: 1.5rem;
  border-radius: 8px;
  width: 90%;
  max-width: 500px;
  color: var(--text-primary);
}

.modal-input,
.modal-textarea {
  display: block;
  width: 95%;
  margin-bottom: 0.5rem;
  padding: 0.5rem;
  border: 1px solid #ccc;
  border-radius: 4px;
  background-color: var(--bg-dark);
  color: var(--text-primary);
}

.modal-textarea {
  min-height: 100px;
  resize: vertical;
}

.modal-actions {
  display: flex;
  justify-content: flex-end;
  gap: 0.5rem;
  margin-top: 1rem;
}

.modal-actions button {
  padding: 0.5rem 1rem;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}
.modal-actions button:first-child {
  background-color: green;
  color: white;
}
.modal-actions button:first-child:hover {
  background-color: darkgreen;
}

.projects-list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.project-item {
  display: flex;
  align-items: center;
  padding: 0.75rem;
  background-color: var(--bg-dark);
  border-radius: 4px;
  border: 1px solid #333;
}

.project-sequence {
  width: 50px;
  text-align: center;
  margin-right: 1rem;
  color: var(--text-secondary);
}

.project-title {
  flex: 1;
  font-weight: bold;
  color: var(--text-primary);
}

.project-description {
  flex: 2;
  color: var(--text-secondary);
  margin-right: 1rem;
}

.edit-input {
  flex: 1;
  padding: 0.5rem;
  margin-right: 0.5rem;
  border: 1px solid #ccc;
  border-radius: 4px;
  background-color: var(--bg-lighter);
}

.edit-textarea {
  flex: 2;
  padding: 0.5rem;
  margin-right: 0.5rem;
  border-radius: 4px;
  min-height: 24px;
  resize: vertical;
  background-color: var(--bg-lighter);
}

.project-actions {
  display: flex;
  gap: 0.5rem;
}

.sequence-edit {
  display: flex;
  align-items: center;
  gap: 0.15rem;
  margin-right: 0.5rem;
}

.sequence-button {
  width: 24px;
  height: 24px;
  background: #666;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
}

.sequence-button:not(:disabled):hover {
  background: #444;
}

.sequence-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.edit-sequence {
  width: 50px;
  padding: 0.25rem;
  border: 1px solid #ccc;
  border-radius: 4px;
  text-align: center;
  background-color: var(--bg-lighter);
}
</style>
