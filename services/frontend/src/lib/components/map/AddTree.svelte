<script lang="ts">
	import Icon from '$lib/icons/MapTreeIcon.svelte';
	import { Control } from 'svelte-maplibre';
	import { addState } from './AddTree.svelte.ts';
	import { onMount } from 'svelte';

	onMount(addState.onMount);
</script>

<Control position="bottom-right">
	<div class="maplibregl-ctrl-group">
		<button type="button" onclick={addState.toggle} title="Add a new tree">
			<Icon />
		</button>
	</div>
</Control>

<style>
	button {
		padding: 4px;
	}
</style>
