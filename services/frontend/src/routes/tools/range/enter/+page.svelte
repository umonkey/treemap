<script lang="ts">
	import NumberInput from '$lib/ui/number-input/NumberInput.svelte';
	import Button from '$lib/ui/button/Button.svelte';
	import MapPreview from './MapPreview.svelte';
	import { RangeEnterState } from './page.svelte.ts';
	import { lang } from './lang';

	const state = new RangeEnterState();
</script>

<svelte:head>
	<title>{lang.title}</title>
</svelte:head>

<div class="range-enter">
	<h1>{lang.title}</h1>
	<p class="intro">{lang.intro}</p>

	<div class="preview-container">
		<MapPreview gcps={state.mapGcpsWithIndex} suggestedLocation={state.suggestedLocation} />
	</div>

	<div class="inputs-list">
		{#each state.gcps as gcp, i}
			<NumberInput
				label={lang.gcpRadiusLabel(gcp.index)}
				hint={`${gcp.lat.toFixed(6)}, ${gcp.lng.toFixed(6)}`}
				value={state.radii[i]}
				min="0"
				step="0.1"
				placeholder="Distance in meters"
				onChange={(val) => state.setRadius(i, val)}
			/>
		{/each}
	</div>

	{#if state.suggestedLocation}
		<div class="result-box">
			<h3>{lang.suggestedLocation}</h3>
			<p>
				Lat: {state.suggestedLocation.lat.toFixed(7)}, Lng: {state.suggestedLocation.lng.toFixed(7)}
			</p>
		</div>
	{/if}

	<div class="actions">
		<Button type="secondary" onClick={state.handleBack}>{lang.back}</Button>
		{#if state.suggestedLocation}
			<Button onClick={state.handleAddTree}>{lang.saveTree}</Button>
		{/if}
	</div>
</div>

<style>
	.range-enter {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
		background: var(--color-dialog-background);
		color: var(--color-dialog-text);
	}

	h1 {
		margin: 0;
		font-size: 1.5rem;
		font-weight: 500;
	}

	.intro {
		margin: 0;
		color: var(--pico-muted-color);
		font-size: 0.95rem;
		line-height: 1.4;
	}

	.preview-container {
		width: 100%;
	}

	.inputs-list {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.result-box {
		padding: 1rem;
		background-color: var(--pico-card-background-color, #f8f9fa);
		border-radius: 8px;
		border: 1px solid var(--pico-muted-border-color, #ccc);

		h3 {
			margin: 0 0 0.5rem 0;
			font-size: 1rem;
			font-weight: 600;
		}

		p {
			margin: 0;
			font-family: monospace;
			font-size: 1.05rem;
		}
	}

	.actions {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-top: 1rem;
	}
</style>
