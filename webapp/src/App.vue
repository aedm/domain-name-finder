<script setup lang='ts'>
import {computed, ref, watchEffect} from 'vue';
import Textbox from '@/components/Textbox.vue';
import {SearchInput, useSearch} from '@/lib/searchHook';
import {asType} from "@/lib/asType";

const words = ref('enter some words here ');
const prefixes = ref('prefix ');
const postfixes = ref('postfix ');
const {setInput, result} = useSearch();
const free = computed(() => result.value?.free);
const reserved = computed(() => result.value?.reserved);

watchEffect(() => {
  setInput({postfixes: postfixes.value, prefixes: prefixes.value, words: words.value});
});

</script>

<template>
  <div class='container mx-auto mt-6'>
    <Textbox v-model='prefixes'/>
    <Textbox v-model='words'/>
    <Textbox v-model='postfixes'/>
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
