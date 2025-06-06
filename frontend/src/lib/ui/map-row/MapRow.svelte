<script lang="ts">
	import type { ILatLng } from '$lib/types';
	import { onMount } from 'svelte';
	import { hooks } from './hooks';

	const { start, end, count } = $props<{
		start: ILatLng;
		end: ILatLng;
		count: number;
	}>();

	const { handleStartChange, handleEndChange, handleCountChange } = hooks({ onMount });

	$effect(() => handleStartChange(start));
	$effect(() => handleEndChange(end));
	$effect(() => handleCountChange(count));
</script>
