<script setup lang="ts">
import { ref, toRefs, PropType, computed, watch, defineEmits } from 'vue';

const props = defineProps<{
  modelValue: string;
}>();

const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void
}>();

const userInput = ref(props.modelValue);
let debounce: (NodeJS.Timeout | undefined) = undefined;

async function performSearch() {
  clearTimeout(debounce!);
  emit('update:modelValue', userInput.value);
}

watch(userInput, () => {
  clearTimeout(debounce!);
  debounce = setTimeout(performSearch, 50)
});

</script>


<template>
  <input
    class='shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline'
    type='text' placeholder='enter search words' v-model='userInput' @keydown.enter='performSearch'
    @keydown.space='performSearch' autofocus />
</template>


<style scoped>

</style>