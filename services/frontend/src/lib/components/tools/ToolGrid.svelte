<script lang="ts">
	import type { Component } from 'svelte';
	import { page } from '$app/state';
	import ChartIcon from '$lib/icons/ChartIcon.svelte';
	import LearnIcon from '$lib/icons/LearnIcon.svelte';
	import MergeIcon from '$lib/icons/MergeIcon.svelte';
	import StreetIcon from '$lib/icons/StreetIcon.svelte';
	import StreetViewIcon from '$lib/icons/StreetViewIcon.svelte';
	import QuestionIcon from '$lib/icons/QuestionIcon.svelte';
	import TriangleIcon from '$lib/icons/TriangleIcon.svelte';
	import UsersIcon from '$lib/icons/UsersIcon.svelte';
	import { hasPermission } from '$lib/stores/authStore';
	import { getLanguageCode } from '$lib/locale';
	import { locale } from './ToolGrid.lang';

	type Tool = {
		name: string;
		href: string;
		icon: Component;
		permission?: string;
	};

	const { onSelect }: { onSelect: () => void } = $props();

	const lang = getLanguageCode();

	const tools: Tool[] = [
		{ name: locale.learn(), href: '/tools/learn', icon: LearnIcon },
		{
			name: locale.trilaterate(),
			href: '/tools/range',
			icon: TriangleIcon,
			permission: 'tree:create'
		},
		{
			name: locale.duplicates(),
			href: '/tools/merge',
			icon: MergeIcon,
			permission: 'tree:delete'
		},
		{
			name: locale.guessSpecies(),
			href: '/tools/unknown-species',
			icon: QuestionIcon,
			permission: 'tree:create'
		},
		{ name: locale.statistics(), href: '/tools/stats', icon: ChartIcon },
		{ name: locale.reports(), href: '/tools/street-report', icon: StreetIcon },
		{
			name: locale.panoramas(),
			href: '/tools/panoramas',
			icon: StreetViewIcon,
			permission: 'pano:edit'
		},
		{ name: locale.users(), href: '/tools/users', icon: UsersIcon, permission: 'user:manage' }
	];

	const visibleTools = $derived(
		tools.filter((tool) => !tool.permission || $hasPermission(tool.permission))
	);

	const isActive = (href: string) => {
		const path = page.url.pathname;
		return path === href || path.startsWith(`${href}/`);
	};
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
