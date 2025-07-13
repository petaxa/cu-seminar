<!--
  Usage:
```md
---
layout: two-cols-header
---
This spans both
::left::
# Left
This shows on the left
::right::
# Right
This shows on the right
::bottom::
This shows at the bottom, aligned to the end (bottom) of the grid
```
-->

<script setup lang="ts">
const props = defineProps({
  class: {
    type: String,
  },
  layoutClass: {
    type: String,
  },
})
</script>

<template>
  <div class="slidev-layout two-cols-header w-full h-full" :class="props.layoutClass">
    <div class="col-header">
      <slot />
    </div>
    <div class="col-body">
      <slot name="body" />
    </div>
    <div class="col-bottom" :class="props.class">
      <slot name="bottom" />
    </div>
  </div>
</template>

<style scoped>
.two-cols-header {
  display: grid;
  grid-template-rows: auto 1fr 0;
  gap: 1rem;
}

.col-header {
  grid-area: 1 / 1 / 2 / 3;
}

.col-body {
  grid-area: 2 / 1 / 3 / 2;
  display: flex;
  flex-direction: column;
  justify-content: center;
}

.col-bottom {
  align-self: end;
  grid-area: 3 / 1 / 3 / 3;
}
</style>
