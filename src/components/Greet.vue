<script setup lang="ts">
import { Ref, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const data: Ref<Response | null> = ref(null);
const name = ref("");

interface Coord {
  lat: number;
  lon: number;
}

interface Weather {
  feels_like: number;
  grnd_level: number;
  humidity: number;
  pressure: number;
  sea_level: number;
  temp: number;
  temp_max: number;
  temp_min: number;
}

interface Wind {
  deg: number;
  speed: number;
}

interface Response {
  coord: Coord;
  main: Weather;
  name: string;
  timezone: number;
  visibility: number;
  wind: Wind;
}

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  data.value = await invoke("get_data", { city: name.value });
}
</script>

<template>
  <form class="row" @submit.prevent="greet">
    <input id="greet-input" v-model="name" placeholder="Enter a name..." />
    <button type="submit">Greet</button>
  </form>

  <p v-if="data">{{ data.main.temp }}</p>
</template>
