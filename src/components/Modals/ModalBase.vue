<template>
  <div class="modal-base fixed flex flex-col w-full h-full">
    <div class="bg-gray-800 w-full h-full absolute opacity-75 z-10"></div>
    <div class="text-black bg-white z-20 py-4 px-20 mx-auto my-auto rounded-md">
      <slot name="header"></slot>
      <slot name="body"></slot>
      <slot name="footer"></slot>
    </div>
  </div>
</template>
<style>
.modal-base {
  z-index: 999;
}
</style>
