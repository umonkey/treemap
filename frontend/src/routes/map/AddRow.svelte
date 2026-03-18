<script lang="ts">
	import CrossHair from '$lib/icons/CrossHair.svelte';
	import Icon from '$lib/icons/Ruler.svelte';
	import Button from '$lib/ui/button/Button.svelte';
	import { onMount } from 'svelte';
	import { GeoJSON, LineLayer, getMapContext } from 'svelte-maplibre';
	import { Control } from 'svelte-maplibre';
	import { addState } from './AddRow.svelte.ts';

	const { map } = getMapContext();

	onMount(() => {
		const handler = () => {
			const center = map?.getCenter();

			if (center) {
				addState.handleMove(center);
			}
		};

		map?.on('move', handler);

		return () => {
			map?.off('move', handler);
		};
	});
</script>

<Control position="top-left">
	<div class="maplibregl-ctrl-group" class:active={addState.active}>
		<button type="button" onclick={addState.toggle} title="Add a new tree">
			<Icon />
		</button>
	</div>
</Control>

{#if addState.active}
	{#if addState.line}
		<GeoJSON data={addState.line}>
			<LineLayer
				layout={{ 'line-cap': 'round', 'line-join': 'round' }}
				paint={{
					'line-color': '#007cbf',
					'line-width': 5,
					'line-opacity': 0.8
				}}
			/>
		</GeoJSON>
	{/if}

	<div class="center">
		<CrossHair />
	</div>

	<div class="panel">
		<Button type="secondary" onClick={addState.handleStart}>|&lt;</Button>
		<Button onClick={addState.handleConfirm}>Add</Button>
		<Button type="secondary" onClick={addState.handleCancel}>Cancel</Button>
		<Button type="secondary" onClick={addState.handleEnd}>&gt;|</Button>
	</div>
{/if}

<style>
	button {
		padding: 4px;
	}

	.active {
		background-color: rgba(0 128 0 / 0.5);
	}

	.center {
		position: absolute;
		left: 50%;
		top: 50%;
		z-index: 10;
		transform: translate(-50%, -50%);
		width: 50px;
		height: 50px;
	}

	.panel {
		position: absolute;
		bottom: 10px;
		left: 50%;
		transform: translate(-50%, 0);

		z-index: 10;
		background-color: #fff;
		padding: 0.5rem;
		border-radius: 5px;
		color: #000;

		display: flex;
		flex-direction: row;
		gap: 1rem;
	}
</style>
