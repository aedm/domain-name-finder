<script setup lang="ts">
import { Triplet } from '@/types';
import { ref, toRefs, PropType } from 'vue';
import Bar from './Bar.vue';

const props = defineProps<{
	triplet: Triplet;
}>();

</script>

<template>
	<div class="main">
		<p>Age group: {{props.triplet.name}}</p>
		<Bar v-for="row in props.triplet.rows" :row="row" />
	</div>
</template>

<style scoped>
.main {
	margin-bottom: 15pt;
}
p {
	color: white;
	text-align: left;
	padding-left: 4pt;
	/* padding-bottom: 1pt; */
}
</style>
