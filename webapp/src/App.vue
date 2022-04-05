<script setup lang='ts'>
import {computed, ref, watchEffect} from 'vue';
import Textbox from '@/components/Textbox.vue';
import {useSearch} from '@/lib/searchHook';
import ButtonGroup from "@/components/ButtonGroup.vue";

const lengthOptions = new Map<number, string>();
lengthOptions.set(1, 'one');
lengthOptions.set(2, 'two');
lengthOptions.set(3, 'three');
const minWordCount = ref(1);
const maxWordCount = ref(2);

const words = ref('enter some words here ');
const prefixes = ref('prefix ');
const postfixes = ref('postfix ');

const {setInput, result} = useSearch();
const free = computed(() => result.value?.free);
const reserved = computed(() => result.value?.reserved);

watchEffect(() => {
  setInput({
    maxWordCount: maxWordCount.value,
    minWordCount: minWordCount.value,
    postfixes: postfixes.value,
    prefixes: prefixes.value,
    words: words.value
  });
});

watchEffect(() => {
  console.log("length", minWordCount.value, maxWordCount.value);
  console.log("ee", typeof minWordCount.value );
});

</script>

<template>
  <div class='container mx-auto mt-6'>
    <div class="my-2">
      At least
      <ButtonGroup v-model="minWordCount" :options="lengthOptions"/>
      and at most
      <ButtonGroup v-model="maxWordCount" :options="lengthOptions"/>
      words.
    </div>

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
