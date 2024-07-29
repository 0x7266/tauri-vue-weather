<script setup lang="ts">
import { Autocomplete } from '../lib/types';
const { suggestions } = defineProps<{ suggestions: Autocomplete[] }>();
</script>

<template>
  <ul v-if="suggestions.length > 0" class="absolute bg-white top-[50px] w-full text-sm flex flex-col border rounded-lg">
    <li v-for="suggestion in suggestions"
      @click="$emit('search', `${suggestion.name},${suggestion.region},${suggestion.country}`)" :key="suggestion.id"
      class="border-b last:border-b-0 hover:bg-neutral-200">
      <p class="py-2 px-3">
        {{ suggestion.name }}, {{ suggestion.region }}, {{ suggestion.country }}
      </p>
    </li>
  </ul>
</template>
