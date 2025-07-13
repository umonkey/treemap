<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import Chart from 'chart.js/auto';

	type Props = {
		type: string;
		// eslint-disable-next-line @typescript-eslint/no-explicit-any
		data: any;
		options?: Chart.ChartOptions;
	};

	const { type, data, options }: Props = $props();

	let canvas;
	let chartInstance;

	// Use onMount to ensure the canvas element is in the DOM
	onMount(() => {
		const ctx = canvas.getContext('2d');

		chartInstance = new Chart(ctx, {
			type,
			data: JSON.parse(JSON.stringify(data)),
			options
		});
	});

	// Use onDestroy to clean up the chart instance when the component is removed
	onDestroy(() => {
		if (chartInstance) {
			chartInstance.destroy();
		}
	});

	// Reactive statement to update the chart when data changes
	$effect(() => {
		if (chartInstance && data) {
			chartInstance.data = JSON.parse(JSON.stringify(data));
			chartInstance.update();
		}
	});
</script>

<canvas bind:this={canvas}></canvas>
