<script lang="ts">
	import Icon from '$lib/icons/LocationIcon.svelte';
	import { Control } from 'svelte-maplibre';
	import { componentState } from './LocateButton.svelte.ts';
</script>

<Control position="top-left">
	<div class="maplibregl-ctrl-group" class:active={componentState.isTracking}>
		<button type="button" onclick={componentState.handleLocate} title="Locate me">
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
