<script setup lang="ts">
import { Day } from '../lib/types';

const { forecast } = defineProps<{ forecast: Day }>()
</script>
<template>
  <div class="flex justify-between text-2xl hover:scale-110 transition-all ease-in-out duration-[100ms] day">
    <span class="font-light">{{ forecast.dt_txt }}</span>
    <div class="flex items-center gap-2">
      <span class="temp">{{ forecast.main.temp }}</span>
      <span class="material-symbols-outlined icon-large">
        <img :src="`https://openweathermap.org/img/wn/${forecast.weather[0].icon}@2x.png`" />
      </span>
    </div>
  </div>
</template>
