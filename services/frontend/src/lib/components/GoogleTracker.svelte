<script lang="ts">
	import { browser } from '$app/environment';
	import { page } from '$app/stores';
	import { config } from '$lib/env';
	import { authStore } from '$lib/stores/authStore';

	const gaId = config.gaMeasurementId;

	$effect(() => {
		if (!browser || !gaId || typeof window.gtag !== 'function') return;

		// Track page views on navigation.
		// Accessing $page.url makes this effect run whenever the URL changes.
		const path = $page.url.pathname + $page.url.search;
		const url = $page.url.href;

		console.debug('[GA] Tracking page view:', path);

		window.gtag('event', 'page_view', {
			page_title: document.title,
			page_location: url,
			page_path: path
		});
	});

	$effect(() => {
		if (!browser || !gaId || typeof window.gtag !== 'function') return;
		if ($authStore?.id) {
			window.gtag('set', { user_id: $authStore.id });
		}
	});
</script>

<svelte:head>
	{#if browser && gaId}
		<script async src="https://www.googletagmanager.com/gtag/js?id={gaId}"></script>
		<script>
			window.dataLayer = window.dataLayer || [];
			window.gtag = function () {
				window.dataLayer.push(arguments);
			};
			window.gtag('js', new Date());
			console.debug('[GA] Initializing for ID:', '{gaId}');

			const gtagConfig = {
				send_page_view: false
			};

			// Check environment from a global or passed through if possible.
			// Since we're in a raw script tag, we can't easily use the 'config' object from TS,
			// but we can check the URL or use a simple heuristic.
			if (window.location.hostname === 'localhost' || window.location.hostname === '127.0.0.1') {
				gtagConfig.debug_mode = true;
				gtagConfig.cookie_domain = 'none';
			}

			window.gtag('config', '{gaId}', gtagConfig);
		</script>
	{/if}
</svelte:head>

<div></div>

<style>
	div {
		display: none;
	}
</style>
