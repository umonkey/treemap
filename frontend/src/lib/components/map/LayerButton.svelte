<script lang="ts">
	import Icon from '$lib/icons/Layers.svelte';
	import { Control } from 'svelte-maplibre';
	import { buttonState } from './LayerButton.svelte.ts';
</script>

<Control position="top-right">
	<div class="maplibregl-ctrl-group" class:active={buttonState.active}>
		<button type="button" title="Switch layers" onclick={buttonState.toggle}>
			<Icon />
		</button>
	</div>
</Control>

<style>
	button {
		padding: 4px;
		color: #000;
	}

	.active {
		background-color: rgba(0 128 0 / 0.5);
	}
</style>
