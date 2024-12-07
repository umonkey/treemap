<script>
	import '@fontsource-variable/inter';

	let { children } = $props();
</script>

<div class="layout">
	<aside>Left sidebar</aside>

	<article>
		{@render children()}
	</article>

	<aside>Right sidebar</aside>
</div>

<style>
	:global {
		:root {
			--gap: 10px;

			font-size: 18px;
			font-family: 'Inter Variable', sans-serif;

			/* Colors */
			--background-color: #fff;
			--icon-color-secondary: #6f869f;
			--link-color: #1083fe;
			--sep-color: #d4dbe2;
			--tab-color: #1083fe;
			--tab-hover-color: #f1f3f5;
			--text-color-inactive: #42576c;
			--text-color: #0b0f14;
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
				--icon-color-secondary: #788ea5;
				--link-color: #208bfe;
				--sep-color: #2e4052;
				--tab-color: #208bfe;
				--text-color-inactive: #aebbc9;
				--text-color: #f1f3f5;
				--tab-hover-color: #1e2936;
			}
		}
	}

	.layout {
		display: flex;
		flex-direction: row;
		margin: 0 auto;
		max-width: 1100px;

		aside {
			width: 250px;
			max-width: 250px;
			text-align: center;
		}

		article {
			max-width: 600px;
			border-left: 1px solid var(--sep-color);
			border-right: 1px solid var(--sep-color);
		}
	}
</style>
