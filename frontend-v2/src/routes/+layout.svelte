<script>
	import '@fontsource-variable/inter';
	import MobileNav from '$lib/components/MobileNav.svelte';
	import LeftSideBar from '$lib/components/LeftSideBar.svelte';
	import RightSideBar from '$lib/components/RightSideBar.svelte';
	import { SvelteToast } from '@zerodevx/svelte-toast';

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

<style>
	:global {
		:root {
			--gap: 10px;

			font-size: 15px;
			font-family: 'Inter Variable', sans-serif;

			/* Colors */
			--background-color: #fff;
			--form-background: #f1f3f5;
			--form-border: #1083fe;
			--icon-color-secondary: #6f869f;
			--link-color: #1083fe;
			--sep-color: #d4dbe2;
			--tab-color: #1083fe;
			--tab-hover-color: #f1f3f5;
			--text-color-inactive: #42576c;
			--text-color: #0b0f14;

			/* Z-indexes */
			--z-map: 2;
			--z-map-preview: 3;
			--z-menu: 6;
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

	@media (prefers-color-scheme: dark) {
		:global {
			:root {
				--background-color: #14191f;
				--form-background: #1e2936;
				--form-border: #208bfe;
				--icon-color-secondary: #788ea5;
				--link-color: #208bfe;
				--sep-color: #2e4052;
				--tab-color: #208bfe;
				--tab-hover-color: #1e2936;
				--text-color-inactive: #aebbc9;
				--text-color: #f1f3f5;
			}
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
		}
	}

	@media (max-width: 400px) {
		article {
			padding-bottom: 50px;
		}
	}
</style>
