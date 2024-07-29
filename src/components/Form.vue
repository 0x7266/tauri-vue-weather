<script setup lang="ts">
import { invoke } from '@tauri-apps/api';
import { reactive } from 'vue';
import { Autocomplete, Data } from '../lib/types';
import { initialDataPlaceholder } from "../lib/utils"
import Suggestions from "./Suggestions.vue"

const search = reactive<{ query: string, timeout: any, suggestions: Autocomplete[], loading: boolean }>({
  query: '',
  timeout: undefined,
  suggestions: [],
  loading: false,
})
let data = reactive<Data>(initialDataPlaceholder)
let input: string;

async function handleInput() {
  search.query = input
  console.log(search.query)
  clearTimeout(search.timeout)
  search.timeout = setTimeout(async () => {
    if (search.query !== "") {
      const response = await invoke<any>("get_autocomplete", { query: search.query });
      search.suggestions = response
    } else {
      search.suggestions = []
    }
  }, 400)
}

async function handleSearch() {
  search.loading = true
  if (search.query !== "") {
    const response = await invoke<any>("get_data", { query: search.query });
    data = await response
  }
  search.loading = false
}
</script>

<template>
  <form @submit.prevent="handleSearch" class="relative w-full">
    <input autocomplete="off" name="query" @input="handleInput" v-model="input"
      class="w-full bg-stone-200 font-semibold font-mono outline-none px-3 py-2 text-lg border-3 rounded-xl"
      :class="{ 'rounded-b-none': data != reactive(initialDataPlaceholder) }" />
    <Suggestions @search="async (query: string) => {
      search.query = query
      await handleSearch()
      search.suggestions = []
    }" :loading="search.loading" :suggestions="search.suggestions" />
  </form>
  <div v-if="data != reactive(initialDataPlaceholder)"
    class="font-mono flex flex-col items-center border-t-2 p-2 font-semibold">
    <p class="text-3xl uppercase">
      {{ data.location.name }}
    </p>
    <p class="flex justify-center gap-2 items-center w-full p-2">
      <span class="text-xl">{{ data.current.temp_c }}</span><img :src="data.current.condition.icon" class="w-[70px]" />
    </p>
  </div>
</template>
