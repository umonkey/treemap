<script lang="ts">
	import { untrack } from 'svelte';
	import { page } from '$app/state';
	import AuthWrapper from '$lib/ui/auth-wrapper/AuthWrapper.svelte';
	import Button from '$lib/ui/button/Button.svelte';
	import Buttons from '$lib/ui/buttons/Buttons.svelte';
	import CheckInput from '$lib/ui/check-input/CheckInput.svelte';
	import Form from '$lib/ui/form/Form.svelte';
	import TextInput from '$lib/ui/text-input/TextInput.svelte';
	import { PageState } from './page.svelte.ts';

	const pageState = new PageState();

	const src = $derived(page.params.src as string);
	const dst = $derived(page.params.dst as string);

	$effect(() => {
		const nextSrc = src;
		const nextDst = dst;
		untrack(() => pageState.init(nextSrc, nextDst));
	});
</script>

<svelte:head>
	<title>Merge duplicate trees</title>
</svelte:head>

<AuthWrapper permission="tree:delete">
	<div class="merge-form">
		<h1>Merge duplicate trees</h1>

		<p>
			Merging marks the source tree as replaced and points it to the target tree. The target tree
			keeps its identity and receives the source tree's data.
		</p>

		<Form onSubmit={pageState.submit}>
			<TextInput
				label="Source tree id"
				type="number"
				value={pageState.src}
				onChange={pageState.setSrc}
			/>
			<TextInput
				label="Target tree id"
				type="number"
				value={pageState.dst}
				onChange={pageState.setDst}
			/>

			<CheckInput
				label="I understand that the source tree will be marked as replaced"
				bind:value={pageState.confirmed}
			/>

			{#if pageState.error}
				<p class="error">{pageState.error.description}</p>
			{/if}

			<Buttons>
				<Button type="secondary" disabled={!pageState.canSwap} onClick={pageState.swap}>
					Swap
				</Button>
				<Button type="danger" disabled={!pageState.canSubmit} onClick={pageState.submit}>
					Merge
				</Button>
				<Button type="cancel" onClick={pageState.cancel}>Cancel</Button>
			</Buttons>
		</Form>
	</div>
</AuthWrapper>

<style>
	.merge-form {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	h1 {
		margin: 0;
		font-size: 1.5rem;
		font-weight: 500;
	}

	p {
		margin: 0;
	}

	.error {
		color: red;
	}
</style>
