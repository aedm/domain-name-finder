<script setup lang='ts'>
import { computed, ref, watch } from 'vue';
import axios, { AxiosResponse } from 'axios';
import Textbox from '@/components/Textbox.vue';
import { search, SearchResult } from '@/lib/search';

const userInput = ref('enter some words here ');
const result = ref(null as SearchResult | null);

// const result = ref(null as AxiosResponse | null);

const free = computed(() => result.value?.free);
const reserved = computed(() => result.value?.reserved);

let requestPromise = null as Promise<SearchResult> | null;

async function doSearch() {
  if (!!requestPromise) return;

  requestPromise = search({ postfixes: '', prefixes: '', words: userInput.value });
  result.value = await requestPromise;
  requestPromise = null;

  // searchInput = userInput.value;
  // const words = searchInput.toLowerCase().split(/[\s,]+/).filter((x) => x != '');
  // const payload = { words };
  // console.log("AXIOS", searchInput);
  // axiosPromise = axios.post('/api/search', payload);
  // result.value = await axiosPromise;
  // axiosPromise = null;
  if (searchInput != userInput.value) setTimeout(doSearch, 0);
}

watch(userInput, () => doSearch());
doSearch();

</script>

<template>
  <div class='container mx-auto mt-6'>
    <Textbox v-model='userInput' />
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
