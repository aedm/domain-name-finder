<script setup lang="ts">
import { Row } from '@/types';
import { ref, toRefs, PropType, computed } from 'vue';

const props = defineProps<{
	row: Row,
}>();

// const sum = computed(() => props.row.bars.reduce((a, b) => a + b, 0));

// const scaledBars = computed(() => {
// 	return props.widths.map(w => [w, w / sum.value)
// });

const count = ref(0);
</script>

<template>
	<div class="main">
		<div class="bars" :style="'width: ' + (props.row.percentage * 90) + '%'" />
		<p>{{ Math.round(props.row.percentage * 1000) / 10 }}%</p>
	</div>
</template>

<style scoped>
div.main {
	height: 10pt;
	display: flex;
	color: white;
	font-size: 12pt;
	margin-bottom: 5pt;
	margin-left: 5pt;
	margin-right: 5pt;
}

div.bars {
	display: flex;
	/* box-shadow: 0px 0px 5px 0px rgba(0, 0, 0, 0.856); */
	border-radius: 2pt;
	min-width: 1px;
	overflow: hidden;
	color: black;
	text-align: right;
	flex-grow: 0;
	margin-bottom: 2px;
	background-color: rgb(0, 255, 179);
}

p {
	color: white;
	padding-left: 5pt;
	margin-top: -4.5pt;
	overflow: visible;
}
</style>
