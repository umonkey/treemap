<script lang="ts">
	import { onMount } from 'svelte';
	import type { ILatLng } from '$lib/types';
	import { hooks } from './hooks';

	const { center } = $props<{ center: ILatLng }>();
	const { handleChange } = hooks(onMount);

	$effect(() => handleChange(center));
</script>
