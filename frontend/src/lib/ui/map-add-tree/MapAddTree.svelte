<script lang="ts">
	import { onMount } from 'svelte';
	import { hooks } from './hooks';
	import type { ILatLng } from '$lib/types';
	import { MapButton } from '$lib/ui';

	const { onConfirm } = $props<{
		onConfirm: (pos: ILatLng) => void;
	}>();

	hooks(onMount, onConfirm);
</script>

<MapButton onClick={() => alert('CLICK')} />
