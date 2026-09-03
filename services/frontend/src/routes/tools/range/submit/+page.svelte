<script lang="ts">
	import Button from '$lib/ui/button/Button.svelte';
	import AuthWrapper from '$lib/ui/auth-wrapper/AuthWrapper.svelte';
	import MapPreview from './MapPreview.svelte';
	import { RangeSubmitState } from './page.svelte.ts';
	import { lang } from './submitLang';

	const pageState = new RangeSubmitState();
</script>

<svelte:head>
	<title>{lang.title}</title>
</svelte:head>

<AuthWrapper permission="tree:create">
	<div class="range-submit">
		<h1>{lang.title}</h1>
		<p class="intro">{lang.intro}</p>

		<div class="preview-container">
			<MapPreview trees={pageState.trees} />
		</div>

		<div class="actions">
			<Button type="secondary" onClick={pageState.handleBack} disabled={pageState.submitting}>
				{lang.back}
			</Button>
			<Button onClick={pageState.handleSubmit} disabled={pageState.submitting}>
				{pageState.submitting ? lang.submitting : lang.submitTrees(pageState.trees.length)}
			</Button>
		</div>
	</div>
</AuthWrapper>

<style>
	.range-submit {
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

	.actions {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-top: 1rem;
	}
</style>
