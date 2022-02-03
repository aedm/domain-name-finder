<script setup lang="ts">
import {ref, watch} from 'vue';

const userInput = ref("enter some nice words here ");
const debouncedInput = ref(userInput.value);

let debounce: (NodeJS.Timeout | undefined) = undefined;

function performSearch() {
  clearTimeout(debounce!);
  debouncedInput.value = userInput.value;
}

function debounceSearch() {
  clearTimeout(debounce!);
  debounce = setTimeout(performSearch, 250);
}

watch(userInput, () => debounceSearch());

</script>

<template>
  <div class="mb-4">
    <input
        class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
        type="text" placeholder="enter search words" v-model="userInput" @keydown.enter='performSearch' @keydown.space='performSearch' autofocus/>
    Search {{ debouncedInput }}
  </div>
</template>
