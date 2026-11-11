<script lang="ts">
	import { setContext, untrack } from 'svelte';
	import { page } from '$app/state';
	import PanoramaHeader from '$lib/components/panoramas/PanoramaHeader.svelte';
	import AuthWrapper from '$lib/ui/auth-wrapper/AuthWrapper.svelte';
	import PageHeader from '$lib/ui/header/PageHeader.svelte';
	import TabList from '$lib/ui/tab-list/TabList.svelte';
	import { locale } from './lang';
	import { PANORAMA_CONTEXT_KEY, PanoramaLayoutState } from './layout.svelte.ts';

	const { children } = $props();

	const state = new PanoramaLayoutState();
	setContext(PANORAMA_CONTEXT_KEY, state);

	const id = $derived(page.params.id as string);
	const pathname = $derived(page.url.pathname);
	const basePath = $derived(`/tools/panoramas/${id}`);

	$effect(() => {
		const cleanup = state.setup();
		untrack(() => state.reload(id));
		return cleanup;
	});

	$effect(() => {
		const status = state.panorama?.status;
		if (status && status !== 'SUCCESS' && status !== 'FAILURE') {
			state.startPolling(id);
		} else {
			state.stopPolling();
		}

		return () => {
			state.stopPolling();
		};
	});

	const tabs = $derived([
		{ title: locale.details(), link: basePath, active: pathname === basePath },
		{
			title: locale.comments(),
			link: `${basePath}/comments`,
			active: pathname === `${basePath}/comments`
		},
		{ title: locale.files(), link: `${basePath}/files`, active: pathname === `${basePath}/files` },
		{ title: locale.edit(), link: `${basePath}/edit`, active: pathname === `${basePath}/edit` },
		{
			title: locale.restart(),
			link: `${basePath}/restart`,
			active: pathname === `${basePath}/restart`
		}
	]);

	const titlePrefix = $derived.by(() => {
		if (pathname === `${basePath}/edit`) return 'Edit Panorama';
		if (pathname === `${basePath}/restart`) return 'Restart Panorama';
		return 'Panorama';
	});
</script>

<svelte:head>
	<title>{titlePrefix}: {state.panorama?.title || id}</title>
</svelte:head>

<AuthWrapper permission="pano:edit">
	<PageHeader text={state.panorama?.title || id} />

	{#if state.panorama}
		<PanoramaHeader
			{id}
			title={state.panorama.title}
			createdAt={state.panorama.created_at}
			status={state.panorama.status}
			bind:visible={state.panorama.visible}
		/>
	{/if}

	<div class="tabs">
		<TabList items={tabs} />
	</div>

	{#if state.error}
		<p class="error" role="alert">Error loading panorama: {state.error.description}</p>
	{/if}

	{@render children()}
</AuthWrapper>

<style>
	.tabs {
		margin-top: 1rem;
	}

	.error {
		color: red;
	}
</style>
