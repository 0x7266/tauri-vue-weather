<script setup lang="ts">
import { invoke } from '@tauri-apps/api';
import { reactive } from 'vue';
import { Autocomplete, Data } from '../lib/types';
import { initialDataPlaceholder } from "../lib/utils"
import Sugestions from "./Sugestions.vue"

const search = reactive<{ query: string, timeout: any, sugestions: Autocomplete[] }>({
  query: '',
  timeout: undefined,
  sugestions: [],
})
let data = reactive<Data>(initialDataPlaceholder)

async function handleInput() {
  console.log(search.query)
  clearTimeout(search.timeout)
  search.timeout = setTimeout(async () => {
    if (search.query !== "") {
      const response = await invoke<any>("get_autocomplete", { query: search.query });
      search.sugestions = response
    } else {
      search.sugestions = []
    }
  }, 400)
}

async function handleSearch() {
  if (search.query !== "") {
    const response = await invoke<any>("get_data", { query: search.query });
    data = await response
  }
}
</script>

<template>
  <form @submit.prevent="handleSearch" class="relative w-full flex flex-col gap-1 p-2">
    <input name="query" @input="handleInput" v-model="search.query"
      class="w-full px-3 py-2 text-lg border border-3 rounded" />
    <Sugestions @search="async (query: string) => {
      search.query = query
      await handleSearch()
    }" :sugestions="search.sugestions" />
  </form>
  <div v-if="data != reactive(initialDataPlaceholder)" class="flex flex-col items-center border-t-2 p-2 font-semibold">
    <p class="">
      {{ data.location.name }}
    </p>
    <p class="flex justify-between items-center w-full p-2">
      <span>{{ data.current.temp_c }}</span><img :src="data.current.condition.icon" class="w-[80px]" />
    </p>
  </div>
</template>
