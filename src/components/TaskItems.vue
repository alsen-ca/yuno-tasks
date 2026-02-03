<template>
  <div class="task-item-delineator space-bellow">
    <small>{{ task?.description }}</small>
    <div class="viewable-header">
        <button @click="$emit('back-to-task')" class="back-button task-item-bck-color">
            <img src="../assets/arrow-big-left.svg" alt="Back to parent Task" class="default-icon" />
        </button>
        <h2 v-if="task" class="task-title task-item-color">Task - {{ task?.title }}</h2>

        <button class="filter-button" @click="handleFilterModal">
            <img src="../assets/filter.svg" alt="Filter Task Items" class="default-icon" />
        </button>
        <button class="add-button" @click="handleCreateModal">
            <img src="../assets/plus.svg" alt="Add Task Item" class="plus-icon" />
        </button>
    </div>
  </div>


  <div v-if="isCreateOpen && isModalOpen" class="modal-overlay" @click.self="handleCloseModal">
    <div class="modal-content">
        <h3>New Task Item</h3>

        <input
          v-model="content"
          placeholder="Content"
          class="modal-input"
        />

        <div class="modal-actions">
            <button @click="createTaskItem">Create</button>
            <button @click="handleCloseModal" class="cancel-button">Cancel</button>
        </div>

        <p v-if="result" class="success-message">{{ result }}</p>
        <p v-if="error" class="error-message">{{ error }}</p>
    </div>
  </div>

  <div v-if="isStatusOpen && isModalOpen" class="modal-overlay" @click.self="handleCloseModal">
    <div class="modal-content">
        <h3>Filter Task Items</h3>

        <label>
          <input type="checkbox" v-model="willFilter.pending" /> Pending
        </label>
        <label>
          <input type="checkbox" v-model="willFilter.completed" /> Completed
        </label>
        <label>
          <input type="checkbox" v-model="willFilter.canceled" /> Canceled
        </label>

        <div class="modal-actions">
            <button @click="handleCloseModal" class="cancel-button">Close</button>
        </div>

        <p v-if="result" class="success-message">{{ result }}</p>
        <p v-if="error" class="error-message">{{ error }}</p>
    </div>
  </div>


    <div class="viewable-list">
      <div v-for="item in filtered_task_items" :key="item.id" class="viewable-item">
        <!-- Sequence -->
        <div v-if="editTaskItemId === item.id" class="sequence-edit-item">
          <button @click="item.sequence--" :disabled="item.sequence <= 1" class="sequence-button" >
            -
          </button>
          <input v-model.number="item.sequence" type="number" min="1" class="edit-sequence" />
          <button @click="item.sequence++" class="sequence-button">
            +
          </button>
        </div>
        <div v-else class="arrangement-sequence">
          {{ item.sequence }}
        </div>

        <!-- Content -->
        <div v-if="editTaskItemId !== item.id" class="arrangement-selectable" @click="startChangingStatus(item.id)" >
          <div class="arrangement-title">{{ item.content }}</div>
        </div>
        <template v-else>
          <input v-model="item.content" class="edit-input" />
        </template>
          <p v-if="item.status !== 'Pending'">({{ item.status }})</p>

        <div class="arrangement-actions">
          <template v-if="statusTaskItemId === item.id">
            <div v-if="item.status === 'Pending'">
              <button class="icon-button" title="Mark as Canceled" @click="changeStatusCanceled(item)">
                <img src="../assets/circle-x.svg"class="default-icon separate-button" />
              </button>
              <button class="icon-button" title="Mark as Completed" @click="changeStatusCompleted(item)">
                <img src="../assets/circle-check.svg"class="default-icon separate-button"/>
              </button>
            </div>
            <div v-else>
              <button class="icon-button" title="Mark as Pending" @click="changeStatusPending(item)">
                <img src="../assets/circle-alert.svg"class="default-icon separate-button task-item-bck-accent-color" />
              </button>
            </div>

          </template>
          <template v-else>
            <template v-if="editTaskItemId === item.id">
              <button @click="saveEdit(item)" class="save-button">Save</button>
              <button @click="cancelEdit" class="cancel-button">Cancel</button>
              <button @click="() => deleteTaskItem(item.id)" class="delete-button">Delete</button>
            </template>
            <template v-else>
              <button @click="startEditing(item.id)" class="edit-button">Edit</button>
            </template>
          </template>

        </div>
      </div>
    </div>
</template>

<script setup>
import { ref, onMounted, computed } from 'vue';
import { invoke } from "@tauri-apps/api/core";
import { useMessages } from '../composables/useMessages';
import { useModal } from '../composables/useModal'

console.log('debug')
const props = defineProps({
  taskId: {
    type: Number,
    required: true,
  },
});
const error = ref(null);
const result = ref(null);
const task = ref(null);
const content = ref(null);
const task_items = ref([]);
const editTaskItemId = ref(null);
const statusTaskItemId = ref(null);
const isCreateOpen = ref(false);
const isStatusOpen = ref(false);
const willFilter = ref({
  pending: true,
  confirmed: false,
  canceled: false
})
const filtered_task_items = computed(() => {
  const items = task_items.value || [];
  return items
    .filter(item => {
      if (willFilter.value.pending && item.status === "Pending") return true;
      if (willFilter.value.completed && item.status === "Completed") return true;
      if (willFilter.value.canceled && item.status === "Canceled") return true;
      return false;
    })
    // Put items without sequence at bottom
    .sort((a, b) => (a.sequence === null) - (b.sequence === null) || (a.sequence || 0) - (b.sequence || 0));
});
console.log("Filtered task items: ", filtered_task_items)


const emit = defineEmits(['back-to-task']);
useMessages(error, result);
const { isModalOpen, openModal, closeModal } = useModal();

onMounted(async () => {
  await fetchTask();
  await fetchTaskItems();
});

function handleCloseModal() {
  closeModal();
  content.value = "";
  isCreateOpen.value = false;
  isStatusOpen.value = false;
}

function handleCreateModal() {
  isCreateOpen.value = true;
  openModal();
  content.value = "";
}

function handleFilterModal() {
  isStatusOpen.value = true;
  openModal();
}

async function fetchTask() {
  try {
    task.value = await invoke("get_task", { id: props.taskId });
  } catch (e) {
    console.error("Failed to fetch task:", e);
  }
}

async function createTaskItem() {
  error.value = null
  result.value = null

  try {
    // taskId = props.taskId
    const itemId = await invoke("create_task_item", {
      content: content.value
    });
    const linkId = await invoke("link_task_item", {
      taskId: props.taskId,
      itemId: itemId
    })
    result.value = `Task Item created with ID ${itemId} and linked with Id: ${linkId}`
    fetchTaskItems();
    closeModal();
  } catch (e) {
    error.value = e
  }
}

async function fetchTaskItems() {
  try {
    task_items.value = await invoke("get_task_items", { taskId: props.taskId });
    console.debug("Task item list: ", task_items)
  } catch (e) {
    console.error("Failed to fetch tasks items:", e);
  }
}

function startEditing(id) {
  editTaskItemId.value = id;
}

function startChangingStatus(id) {
  statusTaskItemId.value = statusTaskItemId.value === id ? null : id;
}

async function saveEdit(task_item) {
  console.log(`Task Item to be updated: ${task_item.id}`)
  try {
    const statusMap = {
      "Pending": 0,
      "Completed": 1,
      "Canceled": 2
    };
    await invoke("update_task_item_content", {
      taskItemId: task_item.id,
      content: task_item.content,
      status: statusMap[task_item.status]
    });
    if (task_item.status === "Pending") {
      await changeSequence(task_item);
    }

    editTaskItemId.value = null;
    await fetchTaskItems();
  } catch (e) {
    error.value = `Failed to update task item: ${e}`;
  }
}

async function cancelEdit() {
  editTaskItemId.value = null;
  await fetchTaskItems();
}

async function deleteTaskItem(id) {
  try {
    await invoke("delete_task_item", { id });
    await fetchTaskItems();
    result.value = `Task Item of ID ${id} deleted`
  } catch (e) {
    error.value = `Failed to delete task item: ${e}`;
    console.error("Deletion error:", e);
  }
}

function changeStatusCanceled(task_item) {
  const updatedTaskItem = {
    ...task_item,
    status: "Canceled"
  };
  saveEdit(updatedTaskItem);
  statusTaskItemId.value = null;
}
function changeStatusCompleted(task_item) {
  const updatedTaskItem = {
    ...task_item,
    status: "Completed"
  };
  saveEdit(updatedTaskItem);
  statusTaskItemId.value = null;
}
async function changeStatusPending(task_item) {
  const updatedTaskItem = {
    ...task_item,
    status: "Pending"
  };
  saveEdit(updatedTaskItem);
  await invoke("link_task_item", {
    taskId: props.taskId,
    itemId: task_item.id
  })
  statusTaskItemId.value = null;
}

async function changeSequence(task_item) {
  console.log(`Changing the sequence for Task Item: ${task_item}`)
  try {
    await invoke("update_task_item_sequence", {
      taskId: props.taskId,
      taskItemId: task_item?.id,
      newSequence: task_item?.sequence
    });
  } catch (e) {
    error.value = `Failed to update task item's sequence: ${e}`;
  }
}
</script>

<style setup>
.task-item-delineator {
  border-top: 2px solid var(--accent-color);
  border-bottom: 2px solid var(--accent-color);
}

.task-item-bck-color {
  background-color: var(--accent-secondary);
}

.task-item-color {
  color: var(--accent-secondary);
}

.space-bellow {
  margin-bottom: 1rem;
}

.separate-button {
  padding: 0.375rem 0.75rem;
}
.separate-button:hower {
  padding: 0.5rem;
}

.status-color {
  color: var(--accent-secondary)
}
</style>