<script lang="ts">
	import { page } from '$app/state';
	import { hasPermission } from '$lib/stores/authStore';
	import { isActiveTool, tools } from './tools';

	const visibleTools = $derived(
		tools.filter((tool) => !tool.permission || $hasPermission(tool.permission))
	);
</script>

<ul>
	{#each visibleTools as tool}
		{@const Icon = tool.icon}
		<li class:active={isActiveTool(tool.href, page.url.pathname)}>
			<span class="icon"><Icon /></span>
			<span class="text">
				<a class="name" href={tool.href}>{tool.name}</a>
				<span class="description">{tool.description}</span>
			</span>
		</li>
	{/each}
</ul>

<style>
	ul {
		display: flex;
		flex-direction: column;
		gap: var(--gap);
		list-style: none;
		padding: 0;
		margin: 0;
	}

	li {
		position: relative;
		display: flex;
		align-items: flex-start;
		gap: var(--gap);
		padding: var(--gap);
		border-radius: 8px;
		cursor: pointer;

		&:hover,
		&.active {
			background-color: var(--sep-color);
		}
	}

	.icon {
		flex: 0 0 auto;
		height: 24px;
		width: 24px;
		position: relative;

		:global(svg) {
			height: 24px;
			width: 24px;
		}
	}

	.text {
		display: flex;
		flex-direction: column;
		gap: calc(var(--gap) / 4);
		min-width: 0;
	}

	.name {
		color: inherit;
		text-decoration: none;

		&::after {
			content: '';
			position: absolute;
			inset: 0;
		}
	}

	.description {
		font-size: 0.8em;
		line-height: 1.4;
		opacity: 0.65;
		overflow-wrap: break-word;
	}
</style>
