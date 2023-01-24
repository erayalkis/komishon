<template>
  <div class="modal-base fixed flex flex-col">
    <div class="bg-black w-full h-full absolute opacity-75 z-10"></div>
    <div class="text-black bg-white z-20 h-1/3 w-1/3 mx-auto my-auto">
      <slot name="header"></slot>
      <slot name="body"></slot>
      <slot name="footer"></slot>
    </div>
  </div>
</template>
<style>
.modal-base {
  z-index: 999;
  width: 100vw;
  height: 100vh;
}
</style>
