<script lang="ts">
	import { LocationPicker } from '$lib/ui';
	import { MapIcon } from '$lib/icons';
	import { locale } from '$lib/locale';

	const { value, hint, label, onChange } = $props<{
		value: number[];
		hint?: string;
		label?: string;
		onChange: (value: number[]) => void;
	}>();

	let showMap = $state<boolean>(false);
	let currentValue = $state<number[]>(value);

	const formatLocation = (value: number[]): string => {
		return `${value[0].toFixed(7)}, ${value[1].toFixed(7)}`;
	};

	const toggleMap = () => {
		showMap = !showMap;
	};

	const round = (value: number): number => {
		return Math.round(value * 10000000) / 10000000;
	};

	const handleMove = (center: number[]) => {
		currentValue = center;
		onChange([round(center[0]), round(center[1])]);
	};
</script>

<div class="input">
	<label>
		<span>{label ?? locale.locationLabel()}</span>

		<div class="group">
			<input type="text" value={formatLocation(currentValue)} readonly={true} />
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
