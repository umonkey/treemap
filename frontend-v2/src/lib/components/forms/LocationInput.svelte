<script lang="ts">
	const { value, hint = undefined } = $props();
	import MapIcon from '$lib/icons/MapIcon.svelte';
	import MapLocationPicker from '$lib/components/forms/MapLocationPicker.svelte';

	let showMap = $state(false);

	const formatLocation = (value: number[]): string => {
		return value.join(', ');
	};

	const toggleMap = () => {
		showMap = !showMap;
	};
</script>

<div class="input">
	<label>
		<span>Location</span>

		<div class="group">
			<input type="text" value={formatLocation(value)} />
			<button type="button" onclick={toggleMap}><MapIcon /></button>
		</div>

		{#if showMap}
			<MapLocationPicker />
		{/if}
	</label>

	{#if hint}
		<div class="hint">{hint}</div>
	{/if}
</div>

<style>
	.group {
		display: flex;
		flex-direction: row;
		gap: var(--gap);
	}

	button {
		background: transparent;
		border: none;
		color: var(--text-color);
		width: 30px;
		display: block;
		cursor: pointer;
	}
</style>
