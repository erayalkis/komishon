<template>
  <div
    class="flex-col absolute bg-gray-200"
    v-if="opened"
    tabindex="-1"
    ref="menu"
    :style="{ top: top, left: left }"
  >
    <div>Item 1</div>
    <div>Item 2</div>
    <div>Item 3</div>
  </div>
</template>
<script setup>
import { ref, nextTick } from "vue";

const opened = ref(false);
const top = ref("0px");
const left = ref("0px");
const menu = ref(null);

const close = () => {
  opened.value = false;
};

const open = (e) => {
  e.preventDefault();
  opened.value = true;

  nextTick(() => {
    setMenu(e.y, e.x);
  });
};

const setMenu = (eleTop, eleLeft) => {
  const largestHeight = window.innerHeight - menu.value.offsetHeight - 25;
  const largestWidth = window.innerWidth - menu.value.offsetWidth - 25;
  if (eleTop > largestHeight) eleTop = largestHeight;
  if (eleLeft > largestWidth) eleLeft = largestWidth;
  top.value = eleTop + "px";
  left.value = eleLeft + "px";

  console.log(top.value, left.value);
};

defineExpose({
  close,
  open,
});
</script>
