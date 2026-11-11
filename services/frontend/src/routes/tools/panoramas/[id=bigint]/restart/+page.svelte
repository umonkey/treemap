<script lang="ts">
	import { getContext } from 'svelte';
	import { page } from '$app/state';
	import Button from '$lib/ui/button/Button.svelte';
	import Buttons from '$lib/ui/buttons/Buttons.svelte';
	import Form from '$lib/ui/form/Form.svelte';
	import CheckInput from '$lib/ui/check-input/CheckInput.svelte';
	import { PANORAMA_CONTEXT_KEY, type PanoramaLayoutState } from '../layout.svelte.ts';
	import { PageState } from './page.svelte.ts';

	const id = $derived(page.params.id as string);
	const layout = getContext<PanoramaLayoutState>(PANORAMA_CONTEXT_KEY);
	const pageState = new PageState();
</script>

<article>
	{#if pageState.error}
		<p class="error">Error: {pageState.error.description}</p>
	{/if}

	{#if layout.panorama}
		<Form onSubmit={() => pageState.submit(id, layout)}>
			<CheckInput
				label="Delete temporary files (uncheck to preserve extracted files and retry with different settings)"
				bind:value={pageState.deleteTemporaryFiles}
			/>
			<CheckInput
				label="I understand that this process will incur additional cost"
				bind:value={pageState.understandCost}
			/>

			<Buttons>
				<Button
					type="danger"
					disabled={!pageState.understandCost || pageState.isSaving}
					onClick={() => pageState.submit(id, layout)}>Restart Panorama</Button
				>
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
		display: flex;
		flex-direction: column;
		gap: 1rem;
		margin-top: 1rem;
	}
</style>
