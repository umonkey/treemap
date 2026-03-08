<script lang="ts">
	import '@fontsource-variable/inter';
	import { type BeforeInstallPromptEvent, pwaStore } from '$lib/stores/pwaStore';
	import { Layout, LocationTracker } from '$lib/ui';
	import { autoStartUpload } from '$lib/upload';
	import { validateStoredToken } from '$lib/utils/auth';
	import { SvelteToast } from '@zerodevx/svelte-toast';
	import { onMount } from 'svelte';

	import GoogleTracker from '$lib/components/GoogleTracker.svelte';

	import '$lib/styles/variables.css';
	import '$lib/styles/colors.css';
	import '$lib/styles/fonts.css';
	import '$lib/styles/defaults.css';

	const { children } = $props();

	onMount(() => {
		validateStoredToken();
		autoStartUpload();

		window.addEventListener('beforeinstallprompt', (e) => {
			e.preventDefault();
			pwaStore.set(e as BeforeInstallPromptEvent);
			console.debug('[pwa] Install event stored.');
		});
	});
</script>

<Layout>
	{@render children()}
</Layout>

<SvelteToast />
<GoogleTracker />
<LocationTracker />
