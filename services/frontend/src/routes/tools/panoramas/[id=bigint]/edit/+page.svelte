<script lang="ts">
	import { getContext } from 'svelte';
	import { page } from '$app/state';
	import Button from '$lib/ui/button/Button.svelte';
	import TextInput from '$lib/ui/text-input/TextInput.svelte';
	import NumberInput from '$lib/ui/number-input/NumberInput.svelte';
	import Form from '$lib/ui/form/Form.svelte';
	import Buttons from '$lib/ui/buttons/Buttons.svelte';
	import { roundOffset } from '$lib/utils/geo';
	import PanoramaAligner from './PanoramaAligner.svelte';
	import { PANORAMA_CONTEXT_KEY, type PanoramaLayoutState } from '../layout.svelte.ts';
	import { PageState } from './page.svelte.ts';

	const id = $derived(page.params.id as string);
	const layout = getContext<PanoramaLayoutState>(PANORAMA_CONTEXT_KEY);
	const pageState = new PageState();

	$effect(() => {
		pageState.init(layout.panorama);
	});
</script>

<article>
	{#if pageState.error}
		<p class="error">Error: {pageState.error.description}</p>
	{/if}

	{#if pageState.draft}
		{@const draft = pageState.draft}

		<Form onSubmit={() => pageState.save(layout)}>
			<TextInput label="Title" value={draft.title} onChange={(v) => (draft.title = v)} />

			<PanoramaAligner
				panoramaId={id}
				bind:latOffset={draft.lat_offset}
				bind:lonOffset={draft.lon_offset}
			/>

			<NumberInput
				label="Latitude Offset"
				value={draft.lat_offset}
				step={0.0000001}
				min={Number.NEGATIVE_INFINITY}
				onChange={(v) => (draft.lat_offset = roundOffset(v))}
			/>

			<NumberInput
				label="Longitude Offset"
				value={draft.lon_offset}
				step={0.0000001}
				min={Number.NEGATIVE_INFINITY}
				onChange={(v) => (draft.lon_offset = roundOffset(v))}
			/>

			<Buttons>
				<Button type="submit" disabled={pageState.isSaving}>Save Changes</Button>
				<Button link="/tools/panoramas/{id}" type="cancel">Cancel</Button>
			</Buttons>
		</Form>
	{/if}
</article>

<style>
	.error {
		color: red;
	}

	article {
		margin-top: 1rem;
	}
</style>
