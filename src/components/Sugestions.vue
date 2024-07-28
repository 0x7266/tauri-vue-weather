<script setup lang="ts">
import { Autocomplete } from '../lib/types';
const { sugestions } = defineProps<{ sugestions: Autocomplete[] }>();
</script>

<template>
  <div v-if="sugestions.length > 0"
    class="relative top-2 w-full text-sm flex flex-col gap-1 border py-2 pb-0 rounded shadow">
    <h1 v-for="sugestion in sugestions"
      @click="$emit('search', `${sugestion.name},${sugestion.region},${sugestion.country}`)" :key="sugestion.id"
      class="border-b p-1">
      {{ sugestion.name }}, {{ sugestion.region }}, {{ sugestion.country }}
    </h1>
  </div>
</template>
