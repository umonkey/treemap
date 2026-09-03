<script lang="ts">
	import NumberInput from '$lib/ui/number-input/NumberInput.svelte';
	import Button from '$lib/ui/button/Button.svelte';
	import AuthWrapper from '$lib/ui/auth-wrapper/AuthWrapper.svelte';
	import MapPreview from './MapPreview.svelte';
	import TreeList from './TreeList.svelte';
	import { RangeEnterState } from './page.svelte.ts';
	import { lang } from './lang';

	const pageState = new RangeEnterState();
</script>

<svelte:head>
	<title>{lang.title}</title>
</svelte:head>

<AuthWrapper permission="tree:create">
	<div class="range-enter">
		<h1>{lang.title}</h1>
		<p class="intro">{lang.intro}</p>

		<div class="preview-container">
			<MapPreview
				gcps={pageState.mapGcpsWithIndex}
				suggestedLocation={pageState.suggestedLocation}
				trees={pageState.trees}
			/>
		</div>

		<TreeList />

		<div class="inputs-list">
			{#each pageState.gcps as gcp, i}
				<NumberInput
					label={lang.gcpRadiusLabel(gcp.label)}
					value={pageState.radii[i]}
					min="0"
					step="0.1"
					placeholder="Distance in meters"
					onChange={(val) => pageState.setRadius(i, val)}
				/>
			{/each}
		</div>

		<div class="actions">
			<Button type="secondary" onClick={pageState.handleBack}>{lang.back}</Button>
			{#if pageState.suggestedLocation}
				<Button onClick={pageState.handleAddTree}>{lang.addTree}</Button>
			{/if}
		</div>
	</div>
</AuthWrapper>

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

	.actions {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-top: 1rem;
	}
</style>
