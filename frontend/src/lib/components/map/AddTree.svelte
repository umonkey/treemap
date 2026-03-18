<script lang="ts">
	import { getMapContext } from 'svelte-maplibre';
	import { Control } from 'svelte-maplibre';
	import Icon from '$lib/icons/MapTreeIcon.svelte';
	import Button from '$lib/ui/button/Button.svelte';
	import { addState } from './AddTree.svelte.ts';
	import CrossHair from '$lib/icons/CrossHair.svelte';

	const { map } = getMapContext();

	const { onConfirm } = $props<{
		onConfirm: () => void;
	}>();

	const handleConfirm = () => {
		const ll = map.getCenter();
		addState.handleConfirm(ll);
	};

	$effect(() => {
		addState.onConfirm = onConfirm;
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
	<div class="center">
		<CrossHair />
	</div>

	<div class="panel">
		<Button onClick={handleConfirm}>Add</Button>
		<Button type="secondary" onClick={addState.handleCancel}>Cancel</Button>
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
