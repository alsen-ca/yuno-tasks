<template>
  <div class="app-container">
    <header class="app-header">
      <h1 class="app-title">Yuno Tasks</h1>
      <button class="menu-button" @click="toggleMenu">
        <img :src="menuIconSrc" alt="Menu" class="menu-icon" />
      </button>
    </header>

    <main class="app-main">
      <Projects
        v-if="currentView === 'projects'"
        @project-selected="openTasks"
      />
      <Tasks
        v-else-if="currentView === 'tasks'"
        :projectId="selectedProjectId"
        @task-selected="openTaskItems"
        @back-to-projects="currentView = 'projects'"
      />
    </main>
    <div
      class="sidebar-overlay"
      :class="{ 'active': isMenuOpen }"
      @click="closeMenu"
    ></div>

    <aside class="sidebar" :class="{ 'active': isMenuOpen }">
      <nav>
        <svg width="92" height="30" viewBox="0 0 92 30" xmlns="http://www.w3.org/2000/svg"
            @click="toggleTheme" :class="{ 'dark-mode': isDarkMode }" class="theme-toggle">
          <!-- Toggle track -->
          <rect x="2" y="2" width="88" height="26" rx="13" fill="#f0f0f0" stroke="#ccc" stroke-width="2"/>

          <image href="./assets/sun.svg" x="10" y="5" width="20" height="20" class="sun-icon"/>

          <image href="./assets/moon.svg" x="62" y="5" width="20" height="20" class="moon-icon"/>
        </svg>
      </nav>
    </aside>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue';
import Projects from './components/Projects.vue';
import Tasks from './components/Tasks.vue';
import menuIconDark from './assets/menu.svg';
import menuIconLight from './assets/menu-light.svg';

const isMenuOpen = ref(false);
const isDarkMode = ref(true);
const currentView = ref('projects'); // Defines main viewable window at 'app-main' 
const selectedProjectId = ref(null);
const selectedTaskId = ref(null);

const menuIconSrc = computed(() =>
  isDarkMode.value ? menuIconDark : menuIconLight
);

function toggleMenu() {
  isMenuOpen.value = !isMenuOpen.value;
}

function closeMenu() {
  isMenuOpen.value = false;
}

function toggleTheme() {
  isDarkMode.value = !isDarkMode.value;
  document.body.classList.toggle('light-mode', !isDarkMode.value);
}

function openTasks(projectId) {
  selectedProjectId.value = projectId;
  currentView.value = 'tasks';
}

function openTaskItems(taskId) {
  selectedTaskId.value = taskId;
  currentView.value = 'tasksItems';
}
</script>

<style>
.projects-section {
  flex: 1;
  padding: 1rem;
}

.tasks-section {
  flex: 1;
  padding: 1rem;
  border-left: 1px solid #333;
}

.app-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
  width: 100vw;
  overflow: hidden;
  position: relative;
}

.app-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem;
  background-color: var(--bg-darker);
  border-bottom: 1px solid #333;
}

.app-title {
  margin: 0;
  font-size: 1.5rem;
  color: var(--text-primary);
}

.menu-button {
  background: none;
  border: none;
  cursor: pointer;
  padding: 0.5rem;
  border-radius: 4px;
  transition: background-color 0.2s;
}
.menu-button:hover {
  background-color: rgba(255, 255, 255, 0.1);
}

.menu-icon {
  width: 24px;
  height: 24px;
}

.app-main {
  flex: 1;
  padding: 1rem;
  overflow-y: auto;
}

.sidebar {
  position: fixed;
  top: 0;
  right: 0;
  width: 33%;
  height: 100%;
  background-color: var(--bg-darker);
  border-left: 1px solid #333;
  transform: translateX(100%);
  transition: transform 0.3s ease;
  z-index: 1001;
  padding: 1rem;
  box-sizing: border-box;
}

.sidebar.active {
  transform: translateX(0);
}

.sidebar-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: var(--overlay-color);
  opacity: 0;
  pointer-events: none;
  transition: opacity 0.3s ease;
  z-index: 1000;
}

.sidebar-overlay.active {
  opacity: 1;
  pointer-events: all;
}

.sidebar nav ul {
  list-style: none;
  padding: 0;
  margin: 0;
}

.sidebar nav li {
  padding: 0.75rem 1rem;
  cursor: pointer;
  border-radius: 4px;
  transition: background-color 0.2s;
}

.sidebar nav li:hover {
  background-color: rgba(255, 255, 255, 0.1);
}

.theme-toggle {
  cursor: pointer;
}

.sun-icon {
  transform: translateX(0);
  opacity: 0;
  transition: opacity 0s linear;
}

.moon-icon {
  transform: translateX(0);
  opacity: 1;
  transition: opacity 0s linear;
}

.theme-toggle:not(.dark-mode) .moon-icon {
  transform: translateX(-15px);
  opacity: 0;
  transition: opacity 0.3s ease, transform 0.3s ease;
}
.theme-toggle:not(.dark-mode) .sun-icon {
  opacity: 1;
  transition-delay: 0.3s;
}

.theme-toggle.dark-mode .sun-icon {
  transform: translateX(15px);
  opacity: 0;
  transition: transform 0.3s ease, opacity 0.3s ease;
}
.theme-toggle.dark-mode .moon-icon {
  opacity: 1;
  transition-delay: 0.3s;
}
</style>
