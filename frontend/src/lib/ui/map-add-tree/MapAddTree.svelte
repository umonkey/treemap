<script lang="ts">
	import { hooks } from './hooks';
	import { onMount } from 'svelte';
	import type { ILatLng } from '$lib/types';
	import { MapButton } from '$lib/ui';
	import ICON from '$lib/assets/tree.svg';

	const { onConfirm } = $props<{
		onConfirm: (pos: ILatLng) => void;
	}>();

	const { handleClick } = hooks({ onMount, onConfirm });
</script>

<MapButton onClick={handleClick} icon={ICON} />
