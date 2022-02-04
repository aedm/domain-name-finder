<script setup lang='ts'>
import { computed, ref, watch } from 'vue';
import axios, { AxiosResponse } from 'axios';

const userInput = ref('enter some words here ');
const debouncedInput = ref(userInput.value);
let searchInput = '';
let axiosPromise;
const result = ref(null as AxiosResponse | null);

function sortList(list?: Array<String>): Array<String> {
  let sorted = list ? [...list] : [];
  sorted.sort();
  return sorted;
}

const free = computed(() => sortList(result.value?.data?.free));
const reserved = computed(() => sortList(result.value?.data?.reserved));

let debounce: (NodeJS.Timeout | undefined) = undefined;

async function doSearch() {
  if (!!axiosPromise) return;
  searchInput = debouncedInput.value;
  const words = searchInput.toLowerCase().split(/[\s,]+/).filter((x) => x != '');
  const payload = { words };
  axiosPromise = axios.post('/api/search', payload);
  result.value = await axiosPromise;
  axiosPromise = undefined;
  if (searchInput != debouncedInput.value) setTimeout(doSearch, 0);
}

async function performSearch() {
  clearTimeout(debounce!);
  debouncedInput.value = userInput.value;
  doSearch();
}

function debounceSearch() {
  clearTimeout(debounce!);
  debounce = setTimeout(performSearch, 200);
}

watch(userInput, () => debounceSearch());

doSearch();

</script>

<template>
  <div class='container mx-auto mt-6'>
    <input
      class='shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline'
      type='text' placeholder='enter search words' v-model='userInput' @keydown.enter='performSearch'
      @keydown.space='performSearch' autofocus />
    <div v-if='free.length > 0' class='mt-6'>
      <p class='mt-6'>Available domains:</p>
      <p v-for='name in free' class='text-xl font-semibold'>
        <span class='mr-1 text-sm text-green-500'>&#10003;</span>{{ name }}.com
      </p>
    </div>
    <div v-if='reserved.length > 0' class='mt-6'>
      <p class='mt-6'>Not available:</p>
      <div v-for='name in reserved' class='px-2 py-1 m-1 rounded bg-gray-300 inline-block'>
        {{ name }}.com
      </div>
    </div>
  </div>
</template>
