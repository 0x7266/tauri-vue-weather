<script setup lang="ts">
import "./index.css"
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
  <div class="flex flex-col w-[300px] rounded-xl gap-12 bg-white shadow-2xl">
    <div
      class="font-bold bg-[url('https://images.unsplash.com/photo-1589066724013-06f34f2cc17c?crop=entropy&cs=tinysrgb&fm=jpg&ixid=MnwzMjM4NDZ8MHwxfHJhbmRvbXx8fHx8fHx8fDE2NzgyOTAyNTM&ixlib=rb-4.0.3&q=80')] bg-cover bg-center grayscale hover:grayscale-0 ease-in-out transition-all text-white rounded-t-md">
      <div class="flex justify-between  p-3">
        <div class="left flex items-center gap-1">
          <span class="material-symbols-outlined icon-small">
            more_horiz
          </span>
          <span class="material-symbols-outlined icon-small">
            4g_mobiledata
          </span>
          <span class="material-symbols-outlined icon-small">
            wifi
          </span>
        </div>
        <div class="middle flex items-center">
          13:53 pm
        </div>
        <div class="right flex items-center gap-1">
          <span>30%</span>
          <span class="material-symbols-outlined icon-small">
            lock
          </span>
        </div>
      </div>
      <div class="flex flex-col items-center">
        <div class="flex items-center gap-2 text-2xl">
          <span class="local">Miami, Florida</span>
          <span class="material-symbols-outlined relative top-[5px]">
            expand_more
          </span>
        </div>
        <div class="flex flex-col">
          <span class="text-5xl after:content-['°'] after:absolute font-extrabold">23</span>
          <span class="font-normal">Cloudy</span>
        </div>
        <span class="material-symbols-outlined text-3xl">
          more_horiz
        </span>
      </div>
    </div>
    <div class="flex flex-col w-5/6 self-center text-gray-500">
      <div class="flex justify-between text-2xl hover:scale-110 transition-all ease-in-out duration-[100ms] day">
        <span class="font-light">Sunday</span>
        <div class="flex items-center gap-2">
          <span class="temp">18°</span>
          <span class="material-symbols-outlined icon-large">
            thunderstorm
          </span>
        </div>
      </div>
      <div class="flex justify-between text-2xl hover:scale-110 transition-all ease-in-out duration-[100ms] day">
        <span class="font-light">Tuesday</span>
        <div class="flex items-center gap-2">
          <span class="temp">20°</span>
          <span class="material-symbols-outlined icon-large">
            cloud
          </span>
        </div>
      </div>
      <div class="flex justify-between text-2xl hover:scale-110 transition-all ease-in-out duration-[100ms] day">
        <span class="font-light">Wednesday</span>
        <div class="flex items-center gap-2">
          <span class="temp">20°</span>
          <span class="material-symbols-outlined icon-large">
            filter_drama
          </span>
        </div>
      </div>
      <div class="flex justify-between text-2xl hover:scale-110 transition-all ease-in-out duration-[100ms] day">
        <span class="font-light">Thursday</span>
        <div class="flex items-center gap-2">
          <span class="temp">17°</span>
          <span class="material-symbols-outlined icon-large">
            air
          </span>
        </div>
      </div>
      <div class="flex justify-between text-2xl hover:scale-110 transition-all ease-in-out duration-[100ms] day">
        <span class="font-light">Friday</span>
        <div class="flex items-center gap-2">
          <span class="temp">15°</span>
          <span class="material-symbols-outlined icon-large">
            severe_cold
          </span>
        </div>
      </div>
    </div>
    <div
      class="text-center font-semibold bg-gray-500 rounded-b-md p-5 text-xl text-white hover:bg-gray-700 transition-all ease-in-out">
      <form class="row" @submit.prevent="get_data">
        <input id="city_name" v-model="city_name" placeholder="Enter a name..." />
        <button type="submit">Search</button>
      </form>
    </div>
  </div>

  <p v-if="data">{{ data.main.temp }}</p>
</template>

<style scoped></style>
