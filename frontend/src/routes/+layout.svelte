<script lang="ts">
	import '@fontsource-variable/inter';
	import { validateStoredToken } from '$lib/utils/auth';
	import { SvelteToast } from '@zerodevx/svelte-toast';
	import { onMount } from 'svelte';
	import { Layout, LocationTracker } from '$lib/ui';

	import GoogleTracker from '$lib/components/GoogleTracker.svelte';

	import '$lib/styles/variables.css';
	import '$lib/styles/colors.css';
	import '$lib/styles/fonts.css';
	import '$lib/styles/defaults.css';

	const { children } = $props();

	onMount(() => {
		validateStoredToken();
	});
</script>

<Layout>
	{@render children()}
</Layout>

<SvelteToast />
<GoogleTracker />
<LocationTracker />
