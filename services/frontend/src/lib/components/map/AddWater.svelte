<script lang="ts">
	import Icon from '$lib/icons/WaterIcon.svelte';
	import { Control } from 'svelte-maplibre';
	import { AddWaterState } from './AddWater.svelte.ts';
	import { hasPermission } from '$lib/stores/authStore';
	import { onMount } from 'svelte';

	let componentState = $state<AddWaterState>();

	onMount(() => {
		componentState = new AddWaterState();
		return componentState.onMount();
	});
</script>

{#if $hasPermission('water:manage')}
	<Control position="bottom-right">
		<div class="maplibregl-ctrl-group">
			<button type="button" onclick={componentState?.toggle} title="Add a new water source">
				<Icon />
			</button>
		</div>
	</Control>
{/if}

<style>
	button {
		padding: 4px;
	}
</style>
