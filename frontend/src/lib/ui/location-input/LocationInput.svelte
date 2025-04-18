<script lang="ts">
	import { LocationPicker } from '$lib/ui';
	import { MapIcon } from '$lib/icons';
	import { locale } from '$lib/locale';

	let { value = $bindable([]), hint = undefined, label = locale.locationLabel() } = $props();

	let showMap = $state(false);

	const formatLocation = (value: number[]): string => {
		return `${value[0].toFixed(7)}, ${value[1].toFixed(7)}`;
	};

	const toggleMap = () => {
		showMap = !showMap;
	};

	const handleMove = (center: number[]) => {
		value = center;
	};
</script>

<div class="input">
	<label>
		<span>{label}</span>

		<div class="group">
			<input type="text" value={formatLocation(value)} readonly={true} />
			<button type="button" onclick={toggleMap}><MapIcon /></button>
		</div>

		{#if showMap}
			<LocationPicker center={value} onMove={handleMove} />
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
		color: var(--icon-color-secondary);
		width: 30px;
		display: block;
		cursor: pointer;
	}
</style>
