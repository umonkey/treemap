<script lang="ts">
	import Icon from '$lib/icons/Layers.svelte';
	import { Control } from 'svelte-maplibre';
	import { buttonState } from './LayerButton.svelte.ts';
</script>

<Control position="top-right">
	<div class="maplibregl-ctrl-group">
		<button class="button" type="button" title="Switch layers" onclick={buttonState.toggle}>
			<Icon />
		</button>
	</div>
</Control>

<style>
	.button {
		padding: 4px;
		color: #000;
	}
</style>
