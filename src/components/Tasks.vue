<template>
    <small>{{ project?.description }}</small>
    <div class="viewable-header">
        <button @click="$emit('back-to-projects')" class="back-button">
            <img src="../assets/arrow-big-left.svg" alt="Back to Projects" class="default-icon" />
        </button>
        <h2 v-if="project" class="task-title">Project - {{ project.title }}</h2>
        <button class="add-button" @click="handleOpenModal">
            <img src="../assets/plus.svg" alt="Add Task" class="plus-icon" />
        </button>
    </div>


    <div v-if="isModalOpen" class="modal-overlay" @click.self="closeModal">
      <div class="modal-content">
          <h3>New Task</h3>

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
              <button @click="createTask">Create</button>
              <button @click="handleCloseModal" class="cancel-button">Cancel</button>
          </div>

          <p v-if="result" class="success-message">{{ result }}</p>
          <p v-if="error" class="error-message">{{ error }}</p>
      </div>
    </div>


    <div class="viewable-list">
      <div v-for="task in tasks" :key="task.id" class="viewable-item">
        <!-- Sequence -->
        <div v-if="editTaskId === task.id" class="sequence-edit-item">
          <button @click="task.sequence--" :disabled="task.sequence <= 1" class="sequence-button" >
            -
          </button>
          <input v-model.number="task.sequence" type="number" min="1" class="edit-sequence" />
          <button @click="task.sequence++" class="sequence-button">
            +
          </button>
        </div>
        <div v-else class="arrangement-sequence">
          {{ task.sequence }}
        </div>

        <!-- Title and Description: Clickable when not in edit mode. Sends emit to parent -->
        <div
          v-if="editTaskId !== task.id"
          class="arrangement-selectable"
          @click="$emit('task-selected', task.id)"
        >
          <div class="arrangement-title">{{ task.title }}</div>
          <div class="arrangement-description">{{ task.description }}</div>
        </div>
        <template v-else>
          <input v-model="task.title" class="edit-input" />
          <textarea v-model="task.description" class="edit-textarea" />
        </template>

        <div class="arrangement-actions">
          <template v-if="editTaskId === task.id">
            <button @click="saveEdit(task)" class="save-button">Save</button>
            <button @click="cancelEdit" class="cancel-button">Cancel</button>
          </template>
          <template v-else>
            <button @click="startEditing(task.id)" class="edit-button">Edit</button>
            <button @click="() => deleteTask(task.id)" class="delete-button">Delete</button>
          </template>
        </div>
      </div>
    </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { invoke } from "@tauri-apps/api/core";
import { useMessages } from '../composables/useMessages';
import { useModal } from '../composables/useModal'

const props = defineProps({
  projectId: {
    type: Number,
    required: true,
  },
});
const error = ref(null);
const result = ref(null);
const project = ref(null);
const title = ref(null);
const description = ref(null);
const tasks = ref([]);
const editTaskId = ref(null);

const emit = defineEmits(['back-to-projects', 'task-selected']);
useMessages(error, result);
const { isModalOpen, openModal, closeModal } = useModal();

onMounted(async () => {
    await fetchProject();
    await fetchTasks();
});


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

async function fetchProject() {
  try {
    project.value = await invoke("get_project", { id: props.projectId });
  } catch (e) {
    console.error("Failed to fetch project:", e);
  }
}

async function createTask() {
  error.value = null
  result.value = null

  try {
    const id = await invoke("create_task", {
      projectId: props.projectId,
      title: title.value,
      description: description.value || null
    });
    result.value = `Project created with ID ${id}`
    fetchTasks();
    closeModal();
  } catch (e) {
    error.value = e
  }
}

async function fetchTasks() {
  try {
    tasks.value = await invoke("get_tasks", { projectId: props.projectId });
  } catch (e) {
    console.error("Failed to fetch tasks:", e);
  }
}


function startEditing(id) {
  editTaskId.value = id;
}

async function saveEdit(task) {
  console.log(`Task: ${task.id}`)
  try {
    await invoke("update_task_content", {
      id: task.id,
      title: task.title,
      description: task.description,
    });
    await invoke("update_task_sequence", {
      id: task.id,
      sequence: task.sequence
    });

    editTaskId.value = null;
    await fetchTasks();
  } catch (e) {
    error.value = `Failed to update task: ${e}`;
  }
}

async function cancelEdit() {
  editTaskId.value = null;
  await fetchTasks();
}

async function deleteTask(id) {
  try {
    await invoke("delete_task", { id });
    await fetchTasks();
    result.value = `Task of ID ${id} deleted`
  } catch (e) {
    error.value = `Failed to delete task: ${e}`;
    console.error("Deletion error:", e);
  }
}
</script>

<style>
.task-title {
    color: var(--accent-color)
}
</style>