<script lang="ts">
	import Icon from '$lib/icons/LocationIcon.svelte';
	import { Control } from 'svelte-maplibre';
	import { componentState } from './LocateButton.svelte.ts';
</script>

<Control position="top-left">
	<div class="maplibregl-ctrl-group">
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
</style>
