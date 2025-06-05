<script lang="ts">
	import { onMount } from 'svelte';
	import { hooks } from './hooks';
	import type { ILatLng } from '$lib/types';

	const { onConfirm } = $props<{
		onConfirm: (pos: ILatLng) => void;
	}>();

	hooks(onMount, onConfirm);
</script>
