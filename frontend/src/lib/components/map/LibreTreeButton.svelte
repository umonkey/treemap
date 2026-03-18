<script lang="ts">
	import { Control } from 'svelte-maplibre';
	import Icon from '$lib/icons/MapTreeIcon.svelte';

	const { onClick } = $props<{
		onClick: () => void;
	}>();
</script>

<Control position="top-left">
	<div class="maplibregl-ctrl-group">
		<button type="button" onclick={onClick} title="Add a new tree">
			<Icon />
		</button>
	</div>
</Control>

<style>
	button {
		padding: 4px;
	}
</style>
