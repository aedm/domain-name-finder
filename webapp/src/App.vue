<script setup lang="ts">
import {ref, watch} from 'vue'

const userInput = ref("fine day hello fish");
const debouncedInput = ref(userInput.value);

let debounce = null;
function debounceSearch() {
  clearTimeout(debounce)
  debounce = setTimeout(() => {
    debouncedInput.value = userInput.value
  }, 250)
}

watch(userInput, (old, n) => {
  debounceSearch();
});

</script>

<template>
  <div class="mb-4">
    <input
        class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
        type="text" placeholder="enter search words" v-model="userInput" />
    Debounced {{ debouncedInput }}
  </div>
</template>
