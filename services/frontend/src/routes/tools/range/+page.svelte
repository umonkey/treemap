<script lang="ts">
	import Button from '$lib/ui/button/Button.svelte';
	import LocationInput from '$lib/ui/location-input/LocationInput.svelte';
	import { RangeSetupState } from './page.svelte.ts';
	import { lang } from './lang';

	const state = new RangeSetupState();
</script>

<svelte:head>
	<title>{lang.title}</title>
</svelte:head>

<div class="range-setup">
	<h1>{lang.title}</h1>
	<p class="intro">{lang.intro}</p>

	<div class="gcp-list">
		{#each state.gcps as gcp, i}
			<LocationInput
				label={lang.gcpLabel(i + 1)}
				value={gcp}
				onChange={(val) => state.setGcp(i, val)}
				onClear={() => state.clearGcp(i)}
			/>
		{/each}
	</div>

	<div class="actions">
		<Button disabled={!state.canContinue} onClick={state.handleContinue}>Continue</Button>
	</div>
</div>

<style>
	.range-setup {
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

	.gcp-list {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.actions {
		display: flex;
		justify-content: flex-end;
		margin-top: 1rem;
	}
</style>
