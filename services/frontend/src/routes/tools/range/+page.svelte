<script lang="ts">
	import Button from '$lib/ui/button/Button.svelte';
	import LocationInput from '$lib/ui/location-input/LocationInput.svelte';
	import AuthWrapper from '$lib/ui/auth-wrapper/AuthWrapper.svelte';
	import MapPreview from './MapPreview.svelte';
	import { RangeSetupState } from './page.svelte.ts';
	import { lang } from './lang';

	const pageState = new RangeSetupState();
</script>

<svelte:head>
	<title>{lang.title}</title>
</svelte:head>

<AuthWrapper permission="tree:create">
	<div class="range-setup">
		<h1>{lang.title}</h1>
		<p class="intro">{lang.intro}</p>

		<div class="preview-container">
			<MapPreview gcps={pageState.gcpsWithLabels} />
		</div>

		<div class="gcp-list">
			{#each pageState.gcps as gcp, i}
				{@const label = String.fromCharCode(65 + i)}
				<LocationInput
					label={lang.gcpLabel(label)}
					value={gcp}
					onChange={(val) => pageState.setGcp(i, val)}
					onClear={() => pageState.clearGcp(i)}
				/>
			{/each}
		</div>

		<div class="actions">
			<Button disabled={!pageState.canContinue} onClick={pageState.handleContinue}>Continue</Button>
		</div>
	</div>
</AuthWrapper>

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

	.preview-container {
		width: 100%;
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
