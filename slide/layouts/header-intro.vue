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
  <div class="slidev-layout header-intro w-full h-full" :class="props.layoutClass">
    <div class="col-header">
      <slot />
    </div>
    <div class="col-content" :class="props.class">
      <slot name="content" />
    </div>
    <!-- <div class="col-bottom" :class="props.class">
      <slot name="bottom" />
    </div> -->
  </div>
</template>

<style scoped>
.header-intro {
  display: grid;
  grid-template-rows: auto 1fr 0;
  grid-template-columns: auto;
}

.col-header {
  grid-area: 1 / 1 / 1 / 1;
}

.col-content {
  grid-area: 2 / 1 / 2 / 1;
  place-self: center;
}

.col-bottom {
  align-self: end;
  grid-area: 3 / 1 / 2 / 1;
}
</style>
