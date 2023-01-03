<template>
  <BaseModal>
    <template #header>
      <div class="flex p-3 items-center justify-between">
        <h1 class="text-3xl text-gray-900">Add a tag</h1>
        <p class="text-3xl mr-3" @click="$emit('closeTagModal')">X</p>
      </div>
    </template>
    <template #body>
      <div class="flex">
        <h1>Pick a colour</h1>
        <input type="color" />
      </div>
    </template>
    <template #footer>
      <button @click="$emit('closeTagModal')">Cancel</button>
      <button>Save</button>
    </template>
  </BaseModal>
</template>
<script setup>
import BaseModal from "./ModalBase.vue";
</script>
