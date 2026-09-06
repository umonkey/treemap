<script lang="ts">
	import { untrack } from 'svelte';
	import { PageState } from './page.svelte.ts';
	import { page } from '$app/state';
	import Breadcrumbs from '$lib/components/admin/Breadcrumbs.svelte';
	import PageHeader from '$lib/ui/header/PageHeader.svelte';
	import AuthWrapper from '$lib/ui/auth-wrapper/AuthWrapper.svelte';
	import Button from '$lib/ui/button/Button.svelte';
	import TextInput from '$lib/ui/text-input/TextInput.svelte';
	import NumberInput from '$lib/ui/number-input/NumberInput.svelte';
	import Form from '$lib/ui/form/Form.svelte';
	import Buttons from '$lib/ui/buttons/Buttons.svelte';
	import PanoramaAligner from './PanoramaAligner.svelte';
	import { roundOffset } from '$lib/utils/geo';

	const id = $derived(page.params.id as string);
	const pageState = new PageState();

	$effect(() => {
		untrack(() => pageState.reload(id));
	});
</script>

<svelte:head>
	<title>Edit Panorama: {pageState.panorama?.title || id}</title>
</svelte:head>

<AuthWrapper permission="pano:edit">
	<PageHeader text={`Edit Panorama: ${pageState.panorama?.title || id}`} />
	<Breadcrumbs
		items={[
			{ label: 'Admin', href: '/admin' },
			{ label: 'Panoramas', href: '/admin/panoramas' },
			{ label: pageState.panorama?.title || id, href: `/admin/panoramas/${id}` },
			{ label: 'Edit' }
		]}
	/>
	<article>
		{#if pageState.error}
			<p class="error">Error: {pageState.error.description}</p>
		{:else if pageState.isLoading}
			<p aria-busy="true">Loading panorama...</p>
		{:else if pageState.panorama}
			<Form onSubmit={() => pageState.save()}>
				<TextInput
					label="Title"
					value={pageState.panorama.title}
					onChange={(v) => (pageState.panorama!.title = v)}
				/>

				<PanoramaAligner
					panoramaId={id}
					bind:latOffset={pageState.panorama.lat_offset}
					bind:lonOffset={pageState.panorama.lon_offset}
				/>

				<NumberInput
					label="Latitude Offset"
					value={pageState.panorama.lat_offset}
					step={0.0000001}
					min={Number.NEGATIVE_INFINITY}
					onChange={(v) => (pageState.panorama!.lat_offset = roundOffset(v))}
				/>

				<NumberInput
					label="Longitude Offset"
					value={pageState.panorama.lon_offset}
					step={0.0000001}
					min={Number.NEGATIVE_INFINITY}
					onChange={(v) => (pageState.panorama!.lon_offset = roundOffset(v))}
				/>

				<Buttons>
					<Button type="submit" disabled={pageState.isSaving}>Save Changes</Button>
					<Button link="/admin/panoramas/{id}" type="cancel">Cancel</Button>
				</Buttons>
			</Form>
		{/if}
	</article>
</AuthWrapper>

<style>
	.error {
		color: red;
	}
</style>
