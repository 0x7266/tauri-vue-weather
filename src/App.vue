<script setup lang="ts">
import { Ref, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { Response } from "./lib/types"

const data: Ref<Response | null> = ref(null);
const city_name = ref("");

async function get_data() {
  data.value = await invoke("get_data", { city: city_name.value });
}
</script>

<template>
  <form class="row" @submit.prevent="get_data">
    <input id="city_name" v-model="city_name" placeholder="Enter a name..." />
    <button type="submit">Search</button>
  </form>

  <p v-if="data">{{ data.main.temp }}</p>
</template>

<style scoped></style>
