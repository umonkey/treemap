<script lang="ts">
	const {
		text,
		button
	}: {
		text: string;
		button?: { label: string; link: string };
	} = $props();
</script>

<header class="page-header">
	<h1>{text}</h1>
	{#if button}
		<a href={button.link} role="button" class="outline">{button.label}</a>
	{/if}
</header>

<style>
	.page-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		gap: 1rem;
		margin-bottom: 0.5rem;
	}

	h1 {
		margin: 0;
		font-weight: 400;
		font-size: 2rem;
		line-height: 1.125;
	}

	[role='button'].outline {
		--background-color: transparent;
		--border-color: var(--map-primary-background);
		--color: var(--map-primary-background);

		display: inline-block;
		padding: 0.5rem 1rem;
		border: 1px solid var(--border-color);
		border-radius: var(--pico-border-radius, 0.25rem);
		background-color: var(--background-color);
		color: var(--color);
		text-decoration: none;
		font-size: 1rem;
		line-height: 1.5;
		cursor: pointer;
		transition:
			background-color 0.2s,
			color 0.2s,
			border-color 0.2s;
	}

	[role='button'].outline:hover {
		--background-color: var(--map-primary-background);
		--color: #fff;
	}
</style>
