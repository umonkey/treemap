<script lang="ts">
	import '@fontsource-variable/inter';
	import { type BeforeInstallPromptEvent, pwaStore } from '$lib/stores/pwaStore';
	import { autoStartUpload } from '$lib/upload';
	import { validateStoredToken } from '$lib/utils/auth';
	import { initBackgroundReminders } from '$lib/utils/notifications';
	import { Toaster } from 'svelte-sonner';
	import { onMount } from 'svelte';

	import GoogleTracker from '$lib/components/GoogleTracker.svelte';
	import Layout from '$lib/components/layout/Layout.svelte';
	import WakeGuard from '$lib/components/screen-lock/WakeGuard.svelte';

	import '$lib/styles/variables.css';
	import '$lib/styles/colors.css';
	import '$lib/styles/fonts.css';
	import '$lib/styles/defaults.css';

	const { children } = $props();

	onMount(() => {
		validateStoredToken();
		autoStartUpload();
		initBackgroundReminders();

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

<WakeGuard />
<Toaster position="bottom-left" richColors closeButton />
<GoogleTracker />
