<script lang="ts">
	import { afterNavigate } from '$app/navigation';
	import { config } from '$lib/env';
	import { authStore } from '$lib/stores/authStore';
	import { onMount } from 'svelte';

	$effect(() => {
		if (config.gaMeasurementId && typeof window.gtag === 'function') {
			window.gtag('set', {
				user_id: $authStore?.id || null
			});
		}
	});

	onMount(async () => {
		if (!config.gaMeasurementId) {
			return;
		}

		window.dataLayer = window.dataLayer || [];

		// Define gtag as a global function if it doesn't exist yet.
		if (typeof window.gtag !== 'function') {
			window.gtag = function gtag(...args) {
				window.dataLayer.push(args);
			};
		}

		window.gtag('js', new Date());

		// eslint-disable-next-line @typescript-eslint/no-explicit-any
		const gtagConfig: Record<string, any> = {};
		if (config.environment === 'development') {
			gtagConfig.debug_mode = true;
			gtagConfig.cookie_domain = 'none';
		}

		if ($authStore?.id) {
			gtagConfig.user_id = $authStore.id;
		}

		window.gtag('config', config.gaMeasurementId, gtagConfig);

		const s = document.createElement('script');
		s.async = true;
		s.src = `https://www.googletagmanager.com/gtag/js?id=${config.gaMeasurementId}`;
		document.head.appendChild(s);
	});

	afterNavigate((navigation) => {
		if (!config.gaMeasurementId || typeof window.gtag !== 'function') {
			return;
		}

		// Track page views on SPA navigation.
		window.gtag('event', 'page_view', {
			page_title: document.title,
			page_location: location.href,
			page_path: (navigation.to?.url.pathname || '') + (navigation.to?.url.search || '')
		});
	});
</script>

<div></div>

<style>
	div {
		display: none;
	}
</style>
