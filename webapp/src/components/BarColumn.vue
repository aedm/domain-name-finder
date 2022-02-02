<script setup lang="ts">
import { Triplet } from '@/types';
import { ref, toRefs, PropType, computed } from 'vue';
import TripleBar from './TripleBar.vue';

const props = defineProps<{
	triplets: Triplet[];
}>();

// const scale = computed(() => {
// 	let max = props.triplets.map(t => t.rows).flat().map(t => t.bars.reduce((a, b) => a + b, 0)).reduce((a, b) => Math.max(a, b), 0);
// 	console.log("SCALE", max);
// 	return 100 / max;
// });

</script>

<template>
	<div class="main">
		<TripleBar
			v-for="triplet in props.triplets"
			:triplet="triplet"
			:figure="'hi'"
		/>
	</div>
</template>

<style scoped>
div.main {
}
</style>
