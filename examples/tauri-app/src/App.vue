<template>
  <div class="center-wrapper">
    <button class="macaron-btn" @click="testShowToast()">show toast</button>
  </div>
</template>

<script setup lang="ts">
import { showToast } from '../../../guest-js'

async function testShowToast() {
  await showToast('This is a toast', 'short')
}
</script>

<style scoped>
.center-wrapper {
  position: fixed;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
}

.macaron-btn {
  background-color: #a3d8f4; /* 马卡龙蓝 */
  border: none;
  padding: 12px 24px;
  border-radius: 12px;
  font-size: 16px;
  font-weight: 600;
  color: #ffffff;
  cursor: pointer;
  box-shadow: 0 4px 8px rgba(163, 216, 244, 0.4);
  transition: all 0.2s ease;
}

.macaron-btn:hover {
  background-color: #8ccdf0;
  box-shadow: 0 6px 12px rgba(140, 205, 240, 0.5);
}

.macaron-btn:active {
  transform: scale(0.97);
}
</style>
