<script lang="ts">
	import { untrack } from 'svelte';
	import { PageState } from './page.svelte.ts';
	import { page } from '$app/state';
	import Breadcrumbs from '$lib/components/admin/Breadcrumbs.svelte';
	import PanoramaHeader from '$lib/components/panoramas/PanoramaHeader.svelte';
	import PageHeader from '$lib/ui/header/PageHeader.svelte';
	import AuthWrapper from '$lib/ui/auth-wrapper/AuthWrapper.svelte';
	import Button from '$lib/ui/button/Button.svelte';
	import Buttons from '$lib/ui/buttons/Buttons.svelte';
	import Form from '$lib/ui/form/Form.svelte';
	import CheckInput from '$lib/ui/check-input/CheckInput.svelte';

	const id = $derived(page.params.id as string);
	const pageState = new PageState();

	$effect(() => {
		untrack(() => pageState.reload(id));
	});
</script>

<svelte:head>
	<title>Restart Panorama: {pageState.panorama?.title || id}</title>
</svelte:head>

<AuthWrapper permission="pano:edit">
	<PageHeader text="Restart Panorama" />
	<Breadcrumbs
		items={[
			{ label: 'Admin', href: '/admin' },
			{ label: 'Panoramas', href: '/admin/panoramas' },
			{ label: pageState.panorama?.title || id, href: `/admin/panoramas/${id}` },
			{ label: 'Restart' }
		]}
	/>

	{#if pageState.panorama}
		<PanoramaHeader
			{id}
			title={pageState.panorama.title}
			createdAt={pageState.panorama.created_at}
			status={pageState.panorama.status}
			bind:visible={pageState.panorama.visible}
		/>
	{/if}

	<article>
		{#if pageState.error}
			<p class="error">Error: {pageState.error.description}</p>
		{:else if pageState.isLoading && !pageState.panorama}
			<p aria-busy="true">Loading panorama...</p>
		{:else if pageState.panorama}
			<Form onSubmit={() => pageState.submit(id)}>
				<p>Select restart options:</p>

				<CheckInput
					label="Erase results"
					hint="Delete generated images and hints"
					bind:value={pageState.eraseResults}
				/>

				<CheckInput
					label="Erase temporary files"
					hint="Delete extracted temporary files (except source video and GPX track)"
					bind:value={pageState.eraseTempFiles}
				/>

				<Buttons>
					<Button type="danger" disabled={pageState.isSaving} onClick={() => pageState.submit(id)}>Restart Panorama</Button>
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

	article {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		margin-top: 1rem;
	}
</style>
