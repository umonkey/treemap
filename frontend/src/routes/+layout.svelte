<script lang="ts">
	import '@fontsource-variable/inter';
	import { validateStoredToken } from '$lib/utils/auth';
	import { SvelteToast } from '@zerodevx/svelte-toast';
	import { onMount } from 'svelte';
	import { Layout } from '$lib/ui';

	import GoogleTracker from '$lib/components/GoogleTracker.svelte';
	import LocationTracker from '$lib/components/map/LocationTracker.svelte';

	import '$lib/styles/variables.css';
	import '$lib/styles/forms.css';
	import '$lib/styles/colors.css';

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

<style>
	:global {
		:root {
			font-size: 15px;
			font-family: 'Inter Variable', sans-serif;
		}

		body {
			background-color: var(--background-color);
			color: var(--text-color);
			padding: 0;
			margin: 0;
		}

		p {
			line-height: 1.5em;
		}
	}

	:global(.padded) {
		padding-left: var(--gap);
		padding-right: var(--gap);
	}
</style>
