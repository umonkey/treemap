<script lang="ts">
	import Icon from '$lib/icons/WaterIcon.svelte';
	import { Control } from 'svelte-maplibre';
	import { addWaterState } from './AddWater.svelte.ts';
	import { onMount } from 'svelte';

	onMount(addWaterState.onMount);
</script>

<Control position="bottom-right">
	<div class="maplibregl-ctrl-group">
		<button type="button" onclick={addWaterState.toggle} title="Add a new water source">
			<Icon />
		</button>
	</div>
</Control>

<style>
	button {
		padding: 4px;
	}
</style>
