<script lang="ts">
	import { pageState } from './page.svelte.ts';
	import { page } from '$app/state';
	import { formatDateTimeISO } from '$lib/utils/strings';
	import Breadcrumbs from '$lib/components/admin/Breadcrumbs.svelte';
	import Button from '$lib/ui/button/Button.svelte';
	import Buttons from '$lib/ui/buttons/Buttons.svelte';

	const id = $derived(page.params.id as string);

	$effect(() => {
		pageState.reload(id);
	});
</script>

<svelte:head>
	<title>Sequence: {pageState.sequence?.title || id}</title>
</svelte:head>

<article>
	<header>
		<h1>Sequence Detail</h1>

		<Breadcrumbs
			items={[
				{ label: 'Admin', href: '/admin' },
				{ label: 'Sequences', href: '/admin/sequences' },
				{ label: id }
			]}
		/>
	</header>

	{#if pageState.error}
		<p class="error">Error loading sequence: {pageState.error.description}</p>
	{/if}

	{#if pageState.isLoading}
		<p aria-busy="true">Loading sequence...</p>
	{:else if pageState.sequence}
		<dl class="details">
			<dt>ID</dt>
			<dd>{pageState.sequence.id}</dd>

			<dt>Captured At</dt>
			<dd>{formatDateTimeISO(pageState.sequence.captured_at)}</dd>

			<dt>Title</dt>
			<dd>{pageState.sequence.title}</dd>

			<dt>Images</dt>
			<dd>{pageState.sequence.image_count}</dd>

			<dt>Status</dt>
			<dd>{pageState.sequence.hidden ? 'Hidden' : 'Visible'}</dd>

			<dt>Latitude Offset</dt>
			<dd>{pageState.sequence.lat_offset}</dd>

			<dt>Longitude Offset</dt>
			<dd>{pageState.sequence.lon_offset}</dd>
		</dl>

		<footer>
			<Buttons>
				<Button link="/admin/sequences/{id}/edit">Edit Sequence</Button>
			</Buttons>
		</footer>
	{/if}
</article>

<style>
	.error {
		color: var(--color-error);
		background-color: var(--color-error-background);
		padding: var(--gap);
		border-radius: 6px;
		margin-bottom: var(--gap);
	}

	.details {
		display: grid;
		grid-template-columns: max-content auto;
		gap: 10px 20px;
		margin: 2rem 0 2rem;
	}
</style>
