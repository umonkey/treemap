<script lang="ts">
	import type { Component } from 'svelte';
	import SidebarSearchInput from '$lib/components/search/SidebarSearchInput.svelte';
	import CloseIcon from '$lib/icons/CloseIcon.svelte';
	import LearnIcon from '$lib/icons/LearnIcon.svelte';
	import MergeIcon from '$lib/icons/MergeIcon.svelte';
	import StreetViewIcon from '$lib/icons/StreetViewIcon.svelte';
	import TriangleIcon from '$lib/icons/TriangleIcon.svelte';
	import UsersIcon from '$lib/icons/UsersIcon.svelte';
	import { locale } from '$lib/locale';
	import { hasPermission } from '$lib/stores/authStore';
	import { isSidebarVisible } from '$lib/stores/mobileSidebarStore';
	import { ToolsSidebarLogic } from './ToolsSidebar.svelte.ts';

	type Tool = {
		name: string;
		href: string;
		icon: Component;
		permission?: string;
	};

	const componentState = new ToolsSidebarLogic();

	const tools: Tool[] = [
		{ name: 'Learn', href: '/learn', icon: LearnIcon },
		{ name: 'Trilaterate', href: '/tools/range', icon: TriangleIcon, permission: 'tree:create' },
		{ name: 'Duplicates', href: '/tools/merge', icon: MergeIcon, permission: 'tree:delete' },
		{ name: 'Panoramas', href: '/admin/panoramas', icon: StreetViewIcon, permission: 'pano:edit' },
		{ name: 'Users', href: '/admin/users', icon: UsersIcon, permission: 'user:manage' },
	];

	const visibleTools = $derived(
		tools.filter((tool) => !tool.permission || $hasPermission(tool.permission))
	);
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<div
	class="overlay"
	class:visible={!!$isSidebarVisible}
	onclick={componentState.handleOverlayClick}
>
	<div class="canvas">
		<div class="mobile-header">
			<button class="close-btn" onclick={componentState.close}>
				<CloseIcon />
			</button>
		</div>

		<div class="search">
			<SidebarSearchInput
				value={componentState.query}
				onInput={componentState.handleInput}
				onSearch={componentState.handleSearch}
				onClose={componentState.handleClose}
			/>
		</div>

		<h2>{locale.toolsHeading()}</h2>

		<div class="grid">
			{#each visibleTools as tool}
				{@const Icon = tool.icon}
				<a
					href={tool.href}
					class:active={componentState.isActive(tool.href)}
					onclick={componentState.close}
				>
					<span class="icon"><Icon /></span>
					<span class="label">{tool.name}</span>
				</a>
			{/each}
		</div>
	</div>
</div>

<style>
	.canvas {
		height: 100dvh;
		width: var(--sidebar-width-desktop);
		position: fixed;
		top: 0;
		left: 0;
		box-sizing: border-box;

		background-color: var(--map-menu-background);

		display: flex;
		flex-direction: column;
		gap: calc(var(--gap) * 2);

		padding: var(--gap);
		padding-bottom: calc(2 * var(--gap));
		border-right: 1px solid var(--sep-color);

		text-align: left;
		font-size: 18px;

		color: var(--text-color);

		h2 {
			font-size: 1.25rem;
			font-weight: 400;
			margin: 0;
			opacity: 0.5;
		}

		.mobile-header {
			display: none;
		}

		.search {
			display: none;
		}

		.grid {
			display: grid;
			grid-template-columns: repeat(4, 1fr);
			gap: var(--gap);
			align-content: start;

			a {
				display: flex;
				flex-direction: column;
				align-items: center;
				gap: calc(var(--gap) / 2);

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
			}

			.label {
				font-size: 14px;
				line-height: 1.2;
				margin-top: 0.5rem;
				opacity: 0.75;
			}
		}
	}

	/** Hide by default on small screens. **/
	@media screen and (max-width: 1023px) {
		.overlay {
			animation: fadeIn 0.2s ease-in-out;
			display: none;
			position: fixed;
			top: 0;
			left: 0;
			width: 100vw;
			height: 100dvh;
			background-color: rgba(0, 0, 0, 0.5);
			backdrop-filter: blur(2px);

			&.visible {
				display: block;
			}
		}

		.canvas {
			width: 100vw;
			height: auto;
			top: auto;
			bottom: 0;
			border-right: none;
			border-radius: 16px 16px 0 0;
			padding: var(--gap);
			padding-bottom: calc(2 * var(--gap));
			animation: slideUp 0.2s ease-out;

			.mobile-header {
				position: absolute;
				top: var(--gap);
				right: var(--gap);
				z-index: 1;
				display: block;

				.close-btn {
					background-color: transparent;
					border: none;
					color: light-dark(black, white);
					width: 30px;
					height: 30px;
					padding: 0 4px;
					cursor: pointer;
					opacity: 0.5;
				}
			}
		}
	}

	@keyframes slideUp {
		from {
			transform: translateY(100%);
		}
		to {
			transform: translateY(0);
		}
	}

	/** Must be below the preview on desktop, above on mobile. **/
	.overlay {
		z-index: var(--z-learn-result);
	}

	@media screen and (min-width: 1024px) {
		.canvas .search {
			display: block;
			padding: 0;
		}

		.overlay {
			z-index: 1;
		}
	}
</style>
