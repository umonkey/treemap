<script lang="ts">
	import { untrack } from 'svelte';
	import { PageState } from './page.svelte.ts';
	import { page } from '$app/state';
	import Breadcrumbs from '$lib/components/admin/Breadcrumbs.svelte';
	import AuthWrapper from '$lib/ui/auth-wrapper/AuthWrapper.svelte';
	import Button from '$lib/ui/button/Button.svelte';
	import TextInput from '$lib/ui/text-input/TextInput.svelte';
	import CheckInput from '$lib/ui/check-input/CheckInput.svelte';
	import Form from '$lib/ui/form/Form.svelte';
	import Buttons from '$lib/ui/buttons/Buttons.svelte';

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
	<article>
		<header>
			<h1>Edit Panorama: {pageState.panorama?.title || id}</h1>
			<Breadcrumbs
				items={[
					{ label: 'Admin', href: '/admin' },
					{ label: 'Panoramas', href: '/admin/panoramas' },
					{ label: id, href: `/admin/panoramas/${id}` },
					{ label: 'Edit' }
				]}
			/>
		</header>

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

				<CheckInput label="Visible on map" bind:value={pageState.panorama.visible} />

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
