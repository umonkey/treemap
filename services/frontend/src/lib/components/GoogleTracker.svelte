<script lang="ts">
	import { browser } from '$app/environment';
	import { page } from '$app/stores';
	import { authStore } from '$lib/stores/authStore';
	import { componentState } from './GoogleTracker.svelte.ts';

	const { gaId } = componentState;
	let initialized = false;

	$effect(() => {
		if (!initialized) {
			componentState.init($authStore?.user?.id);
			initialized = true;
		}

		// Sync user ID if it changes after initialization.
		if ($authStore?.user?.id) {
			componentState.setUserId($authStore.user.id);
		}

		// Track page views on navigation.
		// Accessing $page.url makes this effect run whenever the URL changes.
		const path = $page.url.pathname + $page.url.search;
		const url = $page.url.href;
		const title = browser ? document.title : '';

		componentState.trackPageView(url, path, title);
	});
</script>

<svelte:head>
	{#if browser && gaId}
		<script async src="https://www.googletagmanager.com/gtag/js?id={gaId}"></script>
	{/if}
</svelte:head>

<div></div>

<style>
	div {
		display: none;
	}
</style>
