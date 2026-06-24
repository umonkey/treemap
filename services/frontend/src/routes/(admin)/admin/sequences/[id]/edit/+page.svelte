<script lang="ts">
	import { pageState } from './page.svelte.ts';
	import { page } from '$app/state';
	import Breadcrumbs from '$lib/components/admin/Breadcrumbs.svelte';
	import Button from '$lib/ui/button/Button.svelte';
	import Buttons from '$lib/ui/buttons/Buttons.svelte';
	import TextInput from '$lib/ui/text-input/TextInput.svelte';
	import CheckInput from '$lib/ui/check-input/CheckInput.svelte';
	import NumberInput from '$lib/ui/number-input/NumberInput.svelte';

	const id = $derived(page.params.id as string);

	$effect(() => {
		pageState.reload(id);
	});

	const handleSubmit = (e: SubmitEvent) => {
		e.preventDefault();
		pageState.save(id);
	};

	const avgLat = $derived(
		pageState.sequence ? (pageState.sequence.min_lat + pageState.sequence.max_lat) / 2 : 40.1811 // Default to Yerevan
	);

	const metersPerLat = 111132;
	const latDistance = $derived(`~${(Math.abs(pageState.latOffset) * metersPerLat).toFixed(2)}m`);

	const lonDistance = $derived(
		`~${(Math.abs(pageState.lonOffset) * (111319 * Math.cos((avgLat * Math.PI) / 180))).toFixed(
			2
		)}m`
	);
</script>

<svelte:head>
	<title>Edit Sequence: {pageState.sequence?.title || id}</title>
</svelte:head>

<article>
	<header>
		<h1>Edit Sequence</h1>

		<Breadcrumbs
			items={[
				{ label: 'Admin', href: '/admin' },
				{ label: 'Sequences', href: '/admin/sequences' },
				{ label: id, href: `/admin/sequences/${id}` },
				{ label: 'Edit' }
			]}
		/>
	</header>

	{#if pageState.error}
		<p class="error">Error: {pageState.error.description}</p>
	{/if}

	{#if pageState.isLoading}
		<p aria-busy="true">Loading sequence...</p>
	{:else if pageState.sequence}
		<form onsubmit={handleSubmit} class="edit-form">
			<TextInput
				label="Title"
				value={pageState.title}
				onChange={(v) => (pageState.title = v)}
				placeholder="Sequence Title"
			/>

			<CheckInput label="Hidden" bind:value={pageState.hidden} />

			<NumberInput
				label="Latitude Offset"
				value={pageState.latOffset}
				onChange={(v) => (pageState.latOffset = v)}
				hint={latDistance}
				step="0.000001"
				min="-1"
				max="1"
			/>

			<NumberInput
				label="Longitude Offset"
				value={pageState.lonOffset}
				onChange={(v) => (pageState.lonOffset = v)}
				hint={lonDistance}
				step="0.000001"
				min="-1"
				max="1"
			/>

			<Buttons>
				<Button type="submit" disabled={pageState.isSaving}>
					{pageState.isSaving ? 'Saving...' : 'Save Changes'}
				</Button>
				<Button link="/admin/sequences/{id}" type="cancel">Cancel</Button>
			</Buttons>
		</form>
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

	.edit-form {
		display: flex;
		flex-direction: column;
		gap: 20px;
		max-width: 600px;
	}
</style>
