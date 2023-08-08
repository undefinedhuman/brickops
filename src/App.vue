<template>
  <component :is="layout">
    <router-view />
  </component>
</template>

<script lang="ts" setup>
  import { ref, watch } from "vue";
  import { useRoute } from "vue-router";

  const layout = ref("");
  const route = useRoute();

  watch(
    () => route.meta?.layout,
    async (metaLayout) => {
      if (!metaLayout) return;
      layout.value = metaLayout as string;
    },
    { immediate: true }
  );
</script>
