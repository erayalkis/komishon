<template>
  <BaseModal>
    <template #header>
      <div class="flex p-3 items-center justify-between">
        <h1 class="text-4xl text-gray-900">Add a tag</h1>
        <img
          :src="X"
          class="mr-3 cursor-pointer"
          @click="$emit('closeTagModal')"
        />
      </div>
    </template>
    <template #body>
      <div class="flex-col">
        <div class="flex p-3 items-center justify-between w-96">
          <h1 class="text-3xl">Pick a colour:</h1>
          <input type="color" class="w-24 h-24" />
        </div>
        <div class="flex p-3 items-center justify-between w-96">
          <h1 class="text-3xl">Pick a name:</h1>
          <input class="indent-3 text-gray-900 p-1 bg-gray-200 rounded-md" />
        </div>
      </div>
    </template>
    <template #footer>
      <div class="flex gap-3 justify-center">
        <button
          @click="$emit('closeTagModal')"
          class="w-32 h-24 bg-violet-600 text-white hover:bg-violet-700 transition duration-300 ease-out"
        >
          Cancel
        </button>
        <button
          class="w-32 h-24 bg-violet-600 text-white hover:bg-violet-700 transition duration-300 ease-out"
        >
          Save
        </button>
      </div>
    </template>
  </BaseModal>
</template>
<script setup>
import BaseModal from "./ModalBase.vue";
import X from "@/assets/X.svg";
</script>
