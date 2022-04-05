<script setup lang="ts">
import {ref, computed, watchEffect} from 'vue';

const props = defineProps<{
  modelValue: string | number;
  options: Map<string | number, string>,
}>();

const emit = defineEmits<{
  (e: 'update:modelValue', value: string | number): void
}>();

const value = ref(props.modelValue);
const keys = computed(() => [...props.options.keys()]);

watchEffect(() => value.value = props.modelValue);

function setValue(key: string | number) {
  value.value = key;
  emit('update:modelValue', value.value);
}

</script>

<template>
  <div class="inline-flex rounded-md shadow-sm mx-1" role="group">
    <button v-for="key of keys" type="button"
            class="py-1 px-2 text-sm font-medium border-r border-t border-b border-gray-300 focus:z-10 focus:ring-0 focus:ring-blue-400 dark:bg-gray-700 dark:border-gray-600 dark:text-white dark:hover:text-white dark:hover:bg-gray-600 dark:focus:ring-blue-500 dark:focus:text-white"
            :class="[(key === keys[0]) && 'border-l rounded-l-lg',
            (key === keys[keys.length-1]) && 'rounded-r-lg',
            (key === value) ? 'bg-gray-400 hover:bg-blue-400 text-white hover:text-white focus:text-white' : 'bg-gray-200 text-gray-900 hover:bg-gray-100 focus:text-blue-700 hover:text-blue-700']"
            @click="setValue(key)">
      {{ props.options.get(key) }}
    </button>
  </div>
</template>

