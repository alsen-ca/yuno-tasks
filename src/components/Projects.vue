<template>
  <div class="viewable-container">
    <div class="viewable-header">
      <h2>Projects</h2>
      <button class="add-button" @click="handleOpenModal">
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
              <button @click="handleCloseModal" class="cancel-button">Cancel</button>
          </div>

          <p v-if="result" class="success-message">{{ result }}</p>
          <p v-if="error" class="error-message">{{ error }}</p>
      </div>
    </div>


    <div class="viewable-list">
      <div v-for="project in projects" :key="project.id" class="viewable-item">
        <!-- Sequence -->
        <div v-if="editProjectId === project.id" class="sequence-edit-item">
          <button @click="project.sequence--" :disabled="project.sequence <= 1" class="sequence-button" >
            -
          </button>
          <input v-model.number="project.sequence" type="number" min="1" class="edit-sequence" />
          <button @click="project.sequence++" class="sequence-button">
            +
          </button>
        </div>
        <div v-else class="arrangement-sequence">
          {{ project.sequence }}
        </div>

        <!-- Title and Description: Clickable when not in edit mode. Sends emit to parent -->
        <div
          v-if="editProjectId !== project.id"
          class="arrangement-selectable"
          @click="$emit('project-selected', project.id)"
        >
          <div class="arrangement-title">{{ project.title }}</div>
          <div class="arrangement-description">{{ project.description }}</div>
        </div>
        <template v-else>
          <input v-model="project.title" class="edit-input" />
          <textarea v-model="project.description" class="edit-textarea" />
        </template>

        <div class="arrangement-actions">
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
import { invoke } from "@tauri-apps/api/core";
import { useMessages } from '../composables/useMessages';
import { useModal } from '../composables/useModal';

const title = ref(null);
const description = ref(null);
const result = ref(null);
const error = ref(null);
const projects = ref([]);
const editProjectId = ref(null);

const emit = defineEmits(['project-selected']);

useMessages(error, result);
const { isModalOpen, openModal, closeModal } = useModal();


function handleCloseModal() {
  closeModal();
  title.value = "";
  description.value = "";
}

function handleOpenModal() {
  openModal();
  title.value = "";
  description.value = "";
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
    result.value = `Project of ID ${id} deleted`
  } catch (e) {
    error.value = `Failed to delete project: ${e}`;
    console.error("Deletion error:", e);
  }
}
</script>
