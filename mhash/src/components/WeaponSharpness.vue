<script setup lang="ts">
defineProps(['amounts'])
const names = ['Red', 'Orange', 'Yellow', 'Green', 'Blue', 'White', 'Purple']
const MAX_SHARPNESS = 400
</script>

<template>
  <div class="sharpness">
    <div
      v-for="(_, idx) in Math.min(7, amounts.length)"
      :class="'color-' + idx"
      :key="idx"
      :style="{ width: `${(amounts[idx] / MAX_SHARPNESS) * 100}%` }"
      :title="names[idx] + ': ' + amounts[idx]"
    ></div>
  </div>
</template>

<style scoped>
.sharpness {
  display: flex;
  height: 1.2em;
  width: 400px;

  border: 2px solid #757575;
  padding: 2px;
}

.sharpness > div {
  flex: 0 0 auto;
  display: block;
}

.color-0 {
  background: #c61515;
}

.color-1 {
  background: #f36b1a;
}

.color-2 {
  background: #e6e60f;
}

.color-3 {
  background: #3dda11;
}

.color-4 {
  background: #1064e3;
}

.color-5 {
  background: #ffffff;
}

.color-6 {
  background: #ce0fdc;
}
</style>
