<script setup>
  const props = defineProps({
    clickFunc: {
      type: Function,
      required: true,
    },
  });
</script>

<template>
  <button class="btn btn-outline-danger" @click="clickFunc"><slot /></button>
</template>

<style>
.btn-outline-danger {
  color: white !important;
}
</style>
