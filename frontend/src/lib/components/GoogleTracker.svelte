<script lang="ts">
	import { GTM_ID } from '$lib/env';
	import { onMount } from 'svelte';

	onMount(async () => {
		window.dataLayer = window.dataLayer || [];

		function gtag() {
			window.dataLayer.push(arguments);
		}

		gtag('js', new Date());
		gtag('config', GTM_ID);

		const s = document.createElement('script');
		s.async = true;
		s.src = `https://www.googletagmanager.com/gtag/js?id=${GTM_ID}`;
		document.head.appendChild(s);

		console.debug(`GTM initialized for ${GTM_ID}.`);
	});
</script>

<div></div>
