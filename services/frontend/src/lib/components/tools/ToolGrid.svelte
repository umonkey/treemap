<script lang="ts">
	import { page } from '$app/state';
	import { hasPermission } from '$lib/stores/authStore';
	import { getLanguageCode } from '$lib/locale';
	import { isActiveTool, tools } from './tools';

	const { onSelect }: { onSelect: () => void } = $props();

	const lang = getLanguageCode();

	const visibleTools = $derived(
		tools.filter((tool) => !tool.permission || $hasPermission(tool.permission))
	);

	const isActive = (href: string) => isActiveTool(href, page.url.pathname);
</script>

<div class="grid">
	{#each visibleTools as tool}
		{@const Icon = tool.icon}
		<a href={tool.href} class:active={isActive(tool.href)} onclick={onSelect}>
			<span class="icon"><Icon /></span>
			<span class="label" {lang}>{tool.name}</span>
		</a>
	{/each}
</div>

<style>
	.grid {
		display: grid;
		grid-template-columns: repeat(3, minmax(0, 1fr));
		gap: var(--gap);
		align-content: start;

		a {
			display: flex;
			flex-direction: column;
			align-items: center;
			gap: calc(var(--gap) / 2);

			min-width: 0;
			color: inherit;
			text-decoration: none;
			text-align: center;
			padding: var(--gap) calc(var(--gap) / 2);
			border-radius: 8px;
			cursor: pointer;

			&:hover,
			&.active {
				background-color: var(--sep-color);
			}
		}

		.icon {
			flex: 0 0 auto;
			height: 32px;
			width: 32px;
			position: relative;

			:global(svg) {
				height: 32px;
				width: 32px;
			}
		}

		.label {
			flex: 0 0 auto;
			max-width: 100%;
			min-width: 0;
			font-size: 12px;
			line-height: 1.2;
			margin-top: 0.5rem;
			opacity: 0.75;

			display: -webkit-box;
			-webkit-box-orient: vertical;
			-webkit-line-clamp: 2;
			line-clamp: 2;
			overflow: hidden;
			overflow-wrap: break-word;
			hyphens: auto;
		}
	}
</style>
