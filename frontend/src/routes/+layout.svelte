<script>
	import '@fontsource-variable/inter';
	import { SvelteToast } from '@zerodevx/svelte-toast';

	import MobileNav from '$lib/components/MobileNav.svelte';
	import LeftSideBar from '$lib/components/LeftSideBar.svelte';
	import RightSideBar from '$lib/components/RightSideBar.svelte';
	import MobileSidebar from '$lib/components/MobileSidebar.svelte';
	import GoogleTracker from '$lib/components/GoogleTracker.svelte';
	import LocationTracker from '$lib/components/map/LocationTracker.svelte';

	import '$lib/styles/variables.css';
	import '$lib/styles/forms.css';

	let { children } = $props();
</script>

<div class="layout">
	<LeftSideBar />

	<main>
		<article>
			{@render children()}
		</article>

		<MobileNav />
	</main>

	<RightSideBar />
</div>

<SvelteToast />
<MobileSidebar />
<GoogleTracker />
<LocationTracker />

<style>
	:global {
		:root {
			--gap: 10px;

			font-size: 15px;
			font-family: 'Inter Variable', sans-serif;
		}

		body {
			background-color: var(--background-color);
			color: var(--text-color);
			padding: 0;
			margin: 0;
		}

		a {
			color: var(--link-color);
		}

		p {
			line-height: 1.5em;
		}
	}

	.layout {
		display: flex;
		flex-direction: row;
		margin: 0 auto;
		max-width: calc(600px + 2 * 300px);
		box-sizing: border-box;

		main {
			width: 100%;
			min-height: 100dvh;
		}
	}

	:global(.padded) {
		padding-left: var(--gap);
		padding-right: var(--gap);
	}

	@media (max-width: 480px) {
		article {
			padding-bottom: 50px;
		}
	}
</style>
