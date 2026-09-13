<script lang="ts">
	import MapTreeIcon from '$lib/icons/MapTreeIcon.svelte';
	import Ruler from '$lib/icons/Ruler.svelte';
	import WaterIcon from '$lib/icons/WaterIcon.svelte';
	import { locale } from '$lib/locale';
	import { routes } from '$lib/routes';
	import { hasPermission } from '$lib/stores/authStore';
	import { ContextMenuLogic } from './ContextMenu.svelte.ts';
	import { fade } from 'svelte/transition';
	import '$lib/styles/colors.css';

	const componentState = new ContextMenuLogic();

	$effect(() => componentState.subscribe());
</script>

<svelte:window onkeydown={componentState.handleKeyDown} />

{#if componentState.open}
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<div
		class="canvas"
		in:fade={{ duration: 0 }}
		out:fade={{ duration: 200 }}
		onclick={componentState.handleClose}
	>
		<div class="menu">
			<button
				type="button"
				disabled={!$hasPermission('tree:create')}
				onclick={() => componentState.handleNavigate(routes.addTree())}
			>
				<MapTreeIcon />
				<span>{locale.addTitle()}</span>
			</button>
			<button
				type="button"
				disabled={!$hasPermission('tree:create')}
				onclick={() => componentState.handleNavigate(routes.addRow())}
			>
				<Ruler />
				<span>{locale.addRowTitle()}</span>
			</button>
			<button
				type="button"
				disabled={!$hasPermission('water:manage')}
				onclick={() => componentState.handleNavigate(routes.addWater())}
			>
				<WaterIcon />
				<span>{locale.waterAddTitle()}</span>
			</button>
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
		background-color: rgba(0, 0, 0, 0.5);
		backdrop-filter: blur(2px);
		z-index: var(--z-menu);

		.menu {
			background-color: var(--map-menu-background);
			border-radius: 8px;

			position: fixed;
			top: 50%;
			left: 50%;
			transform: translate(-50%, -50%);

			display: flex;
			flex-direction: column;
			padding: 0;
			white-space: nowrap;

			button {
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
				text-align: left;
				cursor: pointer;
				transition: background-color 0.15s ease;

				&:hover:not(:disabled) {
					background-color: rgba(128, 128, 128, 0.2);
				}

				&:disabled {
					opacity: 0.4;
					cursor: default;
				}

				:global(svg) {
					width: 20px;
					height: 20px;
				}
			}
		}
	}
</style>
