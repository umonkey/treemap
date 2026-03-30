<script lang="ts">
	import { config } from '$lib/env';
	import { onMount } from 'svelte';

	onMount(async () => {
		window.dataLayer = window.dataLayer || [];

		// eslint-disable-next-line @typescript-eslint/no-explicit-any
		function gtag(...args: any[]) {
			window.dataLayer.push(args);
		}

		gtag('js', new Date());
		gtag('config', config.gtmId);

		const s = document.createElement('script');
		s.async = true;
		s.src = `https://www.googletagmanager.com/gtag/js?id=${config.gtmId}`;
		document.head.appendChild(s);

		console.debug(`GTM initialized for ${config.gtmId}.`);
	});
</script>

<div></div>
