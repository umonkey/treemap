<script lang="ts">
	import { onMount } from 'svelte';
	import { hooks } from './hooks';
	import { Button } from '$lib/ui';
	import type { ILatLng } from '$lib/types';

	const { onConfirm } = $props<{
		onConfirm: (start: ILatLng, end: ILatLng) => void;
	}>();

	const { start, distance, handleConfirm } = hooks(onMount);
</script>

{#if $distance !== null}
	<div class="info">
		<div class="distance">Length: {$distance.toFixed(2)} m</div>
		<Button onClick={() => handleConfirm(onConfirm)}>Continue</Button>
	</div>
{:else if $start !== null}
	<div class="info">
		<div class="distance">Move the map to send the end of the row.</div>
	</div>
{/if}

<style>
	.info {
		position: absolute;
		bottom: 10px;
		left: 50%;
		z-index: 2;
		transform: translate(-50%, 0);

		display: flex;
		flex-direction: row;
		gap: 10px;
		align-items: center;

		color: #000;
		border-radius: 5px;
		padding: 10px 20px;
		background-color: rgba(255, 255, 255, 0.75);
	}
</style>
