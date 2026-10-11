<script lang="ts">
	import {
		BaseContextMenuLogic,
		type IContextMenuItem,
		type IContextMenuPosition
	} from './BaseContextMenu.svelte.ts';
	import { fade } from 'svelte/transition';
	import '$lib/styles/colors.css';

	const {
		open,
		items,
		position,
		onClose
	}: {
		open: boolean;
		items: IContextMenuItem[];
		position?: IContextMenuPosition;
		onClose: () => void;
	} = $props();

	const logic = new BaseContextMenuLogic();

	$effect(() => {
		logic.position = position;
	});
</script>

<svelte:window
	onkeydown={(e) => logic.handleKeyDown(e, onClose)}
	bind:innerWidth={logic.viewport.width}
	bind:innerHeight={logic.viewport.height}
/>

{#if open}
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<div
		class="canvas"
		onclick={onClose}
		oncontextmenu={(e) => logic.handleBackdropContextMenu(e, onClose)}
	>
		<div
			class="menu"
			class:centered={!position}
			out:fade={{ duration: 100 }}
			bind:clientWidth={logic.menuSize.width}
			bind:clientHeight={logic.menuSize.height}
			style:left={position ? `${logic.clampedPosition?.x ?? 0}px` : undefined}
			style:top={position ? `${logic.clampedPosition?.y ?? 0}px` : undefined}
		>
			{#each items as item}
				{@const Icon = item.icon}
				{#if item.href}
					<a
						href={item.href}
						download={item.download || undefined}
						onclick={item.onClick}
						class:separator={item.separator}
					>
						<Icon />
						<span>{item.label}</span>
					</a>
				{:else}
					<button
						type="button"
						disabled={item.disabled}
						onclick={item.onClick}
						class:separator={item.separator}
					>
						<Icon />
						<span>{item.label}</span>
					</button>
				{/if}
			{/each}
		</div>
	</div>
{/if}

<style>
	.canvas {
		position: fixed;
		top: 0;
		left: 0;
		height: 100%;
		width: 100%;
		z-index: var(--z-menu);
		-webkit-user-select: none;
		user-select: none;
		-webkit-touch-callout: none;

		.menu {
			background-color: var(--map-menu-background);
			border-radius: 8px;
			box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);

			position: fixed;

			display: flex;
			flex-direction: column;
			padding: 0;
			white-space: nowrap;

			button,
			a {
				display: flex;
				flex-direction: row;
				align-items: center;
				gap: var(--gap);
				width: 100%;
				margin: 0;
				padding: 10px 20px;
				background: transparent;
				color: inherit;
				border: none;
				border-radius: 0;
				box-shadow: none;
				box-sizing: border-box;
				text-align: left;
				text-decoration: none;
				cursor: pointer;
				transition: background-color 0.15s ease;
				font-size: 1rem;

				&:hover:not(:disabled) {
					background-color: rgba(128, 128, 128, 0.2);
				}

				&.separator {
					margin-top: 5px;
					border-top: 1px solid rgba(128, 128, 128, 0.2);
				}

				:global(svg) {
					width: 20px;
					height: 20px;
				}
			}

			button:disabled {
				opacity: 0.4;
				cursor: default;
			}
		}

		.menu.centered {
			top: 50%;
			left: 50%;
			transform: translate(-50%, -50%);
		}
	}
</style>
