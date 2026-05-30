<script lang="ts">
	import { locale } from '$lib/locale';
	import { pageState } from './page.svelte';
	import Button from '$lib/ui/button/Button.svelte';
	import Buttons from '$lib/ui/buttons/Buttons.svelte';
	import LocationIcon from '$lib/icons/LocationIcon.svelte';
	import Ruler from '$lib/icons/Ruler.svelte';
	import CloseIcon from '$lib/icons/CloseIcon.svelte';
	import { mapState } from '$lib/components/map/MapLibre.svelte.ts';
	import { componentState } from '$lib/components/map/NearestTree.svelte.ts';
	import { formatMeters } from '$lib/utils/trees';
	import { mapMode } from '$lib/stores/mapMode';
	import { onMount } from 'svelte';
	import '$lib/styles/variables.css';

	onMount(() => {
		$mapMode = 'add';
		return () => {
			$mapMode = undefined;
		};
	});
</script>

<svelte:head>
	<title>{locale.addTitle()} — {locale.appTitle()}</title>
</svelte:head>

<div class="panel">
	<div class="header">
		<div class="title">
			{locale.addTitle()}
		</div>
		<button class="close" onclick={pageState.handleCancel}><CloseIcon /></button>
	</div>

	<div class="props">
		<div class="line">
			<div class="icon">
				<LocationIcon />
			</div>
			<div class="value">
				{mapState.center.lat.toFixed(6)}, {mapState.center.lng.toFixed(6)}
			</div>
		</div>

		{#if componentState.nearest}
			<div class="line">
				<div class="icon">
					<Ruler />
				</div>
				<div class="value">
					{locale.distanceLabel(formatMeters(componentState.nearest.distance))}
				</div>
			</div>
		{/if}
	</div>

	<Buttons>
		<Button onClick={pageState.handleConfirm} disabled={pageState.saving} nowrap>
			{locale.addButton()}
		</Button>
		<Button type="secondary" onClick={pageState.handleQuickAdd} disabled={pageState.saving} nowrap>
			{locale.addQuickAddButton()}
		</Button>
	</Buttons>
</div>

<style>
	.panel {
		z-index: 2;

		display: flex;
		flex-direction: column;
		gap: var(--gap);

		padding: var(--gap);
		line-height: 1.5em;

		position: fixed;
		bottom: 0px;

		width: 100%;
		min-height: 132px;
		box-sizing: border-box;
		background-color: var(--map-menu-background);
		border-top-left-radius: 8px;
		border-top-right-radius: 8px;
		border-right: 1px solid var(--color-dialog-border);

		.header {
			display: flex;
			flex-direction: row;

			.close {
				flex-basis: 30px;
				flex-grow: 0;
				flex-shrink: 0;

				width: 30px;
				height: 30px;
				cursor: pointer;

				background-color: transparent;
				border: none;
				color: light-dark(black, white);
				opacity: 0.5;
			}
		}

		.title {
			flex-grow: 1;
			flex-shrink: 1;
			font-size: 120%;
			line-height: 30px;

			white-space: nowrap;
			overflow: hidden;
			text-overflow: ellipsis;
		}

		.props {
			opacity: 0.7;
			display: flex;
			flex-direction: column;
			gap: 5px;

			.line {
				white-space: nowrap;
				overflow: hidden;
				text-overflow: ellipsis;

				display: flex;
				flex-direction: row;
				align-items: center;
				gap: var(--gap);

				.icon {
					width: 20px;
					height: 20px;
				}
			}
		}
	}

	/* Desktop */
	@media (min-width: 1024px) {
		.panel {
			top: 0;
			left: 0;
			width: 300px;
			height: 100vh;
			border-radius: 0px;
			border-right: 1px solid var(--sep-color);

			.header {
				margin-bottom: var(--gap);
			}
		}
	}

	/* Mobile */
	@media screen and (max-width: 1023px) {
		.panel {
			bottom: var(--bottom-nav-height);
			border-width: 0;
			animation: slideUp 0.2s ease-out;
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
</style>
