<template>
  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 14 14">
    <g>
      <path
        d="M12.5,12.5a1,1,0,0,1-1,1h-9a1,1,0,0,1-1-1V1.5a1,1,0,0,1,1-1h5l5,5Z"
        fill="none"
        :stroke="color"
        stroke-linecap="round"
        stroke-linejoin="round"
      ></path>
      <polyline
        points="7.5 0.5 7.5 5.5 12.5 5.5"
        fill="none"
        :stroke="color"
        stroke-linecap="round"
        stroke-linejoin="round"
      ></polyline>
    </g>
  </svg>
</template>
<script setup>
defineProps({
  color: {
    type: String,
    default: "#000000",
  },
});
</script>
<style>
svg {
  stroke: red;
}
</style>
