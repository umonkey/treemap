<script lang="ts">
	interface BreadcrumbItem {
		label: string;
		href?: string;
	}

	const { items }: { items: BreadcrumbItem[] } = $props();
</script>

<nav aria-label="breadcrumb">
	<ul>
		{#each items as item, i}
			<li>
				{#if item.href && i < items.length - 1}
					<a href={item.href}>{item.label}</a>
				{:else}
					{item.label}
				{/if}
			</li>
		{/each}
	</ul>
</nav>

<style>
	nav[aria-label='breadcrumb'] {
		background-color: rgba(128, 128, 128, 0.1);
		padding: 0 0.5rem;
		border-radius: 0.25rem;
		margin-bottom: var(--pico-spacing);
	}

	nav[aria-label='breadcrumb'] ul {
		display: flex;
		list-style: none;
		margin: 0;
		padding: 0;
	}

	nav[aria-label='breadcrumb'] ul li {
		display: flex;
		align-items: center;
		padding: 0;
	}

	nav[aria-label='breadcrumb'] ul li:not(:last-child)::after {
		content: '>';
		margin: 0 calc(1rem * 0.5);
		color: var(--pico-muted-color);
		opacity: 0.5;
	}

	nav[aria-label='breadcrumb'] a {
		text-decoration: none;
	}
</style>
