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
	aside {
		flex-basis: 300px;
		flex-shrink: 0;
		flex-grow: 0;

		padding: calc(2 * var(--gap));
		box-sizing: border-box;
		border-left: 1px solid var(--sep-color);

		text-align: center;
	}

	.meta {
		font-size: 14px;
	}

	@media (max-width: 480px) {
		aside {
			display: none;
		}
	}
</style>
