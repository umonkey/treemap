<script lang="ts">
	import { browser } from '$app/environment';
	import { afterNavigate } from '$app/navigation';
	import { authStore } from '$lib/stores/authStore';
	import { setSentryUser } from '$lib/utils/sentryUser';
	import { onMount, tick } from 'svelte';
	import { get } from 'svelte/store';
	import { componentState } from './GoogleTracker.svelte.ts';

	const { gaId } = componentState;

	onMount(() => {
		// One-shot setup. The stored session is read here, not subscribed to.
		componentState.init(get(authStore)?.user?.id);

		// Keep both analytics destinations on the same authStore transition: this
		// fires immediately, then on login, token refresh, profile edit and sign-out.
		return authStore.subscribe((auth) => {
			componentState.setUserId(auth?.user?.id);
			setSentryUser(auth?.user);
		});
	});

	// Fires on the initial load and on every client-side navigation.
	afterNavigate(({ to }) => {
		if (!to) return;

		const path = to.url.pathname + to.url.search;
		const url = to.url.href;

		// Wait for the target page's <svelte:head> title to be applied.
		tick().then(() => componentState.trackPageView(url, path, document.title));
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
