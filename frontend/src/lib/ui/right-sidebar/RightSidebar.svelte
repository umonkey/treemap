<aside class="right">
	{#if false}
		<div class="meta">
			<a href="/">Feedback</a>
			<span>&middot;</span>
			<a href="/">Privacy</a>
			<span>&middot;</span>
			<a href="/">Terms</a>
			<span>&middot;</span>
			<a href="/">Help</a>
		</div>
	{/if}
</aside>

<style>
	/* Hide by default */
	aside {
		display: none;
	}

	/* Show on desktop */
	@media (min-width: 1024px) {
		aside {
			display: block;
			flex-basis: 300px;
			flex-shrink: 0;
			flex-grow: 0;

			padding: 0;
			box-sizing: border-box;
			border-left: 1px solid var(--sep-color);

			text-align: center;
		}

		.meta {
			font-size: 14px;
		}
	}
</style>
