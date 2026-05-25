<script lang="ts">
	import { locale } from '$lib/locale';
	import { pageState } from './page.svelte';
	import Button from '$lib/ui/button/Button.svelte';
	import Buttons from '$lib/ui/buttons/Buttons.svelte';
	import CloseIcon from '$lib/icons/CloseIcon.svelte';
	import LineStartIcon from '$lib/icons/LineStartIcon.svelte';
	import LineEndIcon from '$lib/icons/LineEndIcon.svelte';
	import { mapMode } from '$lib/stores/mapMode';
	import { onMount } from 'svelte';
	import RowSizeInput from '$lib/components/map/RowSizeInput.svelte';
	import { mapRowState } from '$lib/stores/mapRowState.svelte';
	import { getDistance } from '$lib/utils';
	import { get } from 'svelte/store';
	import { mapStore } from '$lib/stores/mapStore';
	import '$lib/styles/variables.css';

	const distance = $derived(
		mapRowState.pointA && mapRowState.pointB
			? getDistance(mapRowState.pointA, mapRowState.pointB)
			: 0
	);

	onMount(() => {
		$mapMode = 'add-row';
		mapRowState.init(get(mapStore)?.center);
		return () => {
			$mapMode = undefined;
			mapRowState.init();
		};
	});
</script>

<svelte:head>
	<title>{locale.addTitle()} — {locale.appTitle()}</title>
</svelte:head>

<div class="panel">
	<div class="header">
		<div class="title">
			{locale.addRowTitle()}
		</div>
		<button class="close" onclick={pageState.handleCancel}><CloseIcon /></button>
	</div>

	<div class="props">
		<div class="line">
			<div class="icon">
				<LineStartIcon />
			</div>
			<div class="value">
				{#if mapRowState.pointA}
					{mapRowState.pointA.lat.toFixed(6)}, {mapRowState.pointA.lng.toFixed(6)}
				{:else}
					{locale.addRowStartPoint()}
				{/if}
			</div>
			<Button type="secondary" onClick={pageState.setPointA} nowrap>
				{locale.setPoint()}
			</Button>
		</div>

		<div class="line">
			<div class="icon">
				<LineEndIcon />
			</div>
			<div class="value">
				{#if mapRowState.pointB}
					{mapRowState.pointB.lat.toFixed(6)}, {mapRowState.pointB.lng.toFixed(6)}
				{:else}
					{locale.addRowEndPoint()}
				{/if}
			</div>
			<Button type="secondary" onClick={pageState.setPointB} nowrap>
				{locale.setPoint()}
			</Button>
		</div>

		{#if mapRowState.pointA && mapRowState.pointB}
			<RowSizeInput value={mapRowState.count} {distance} onChange={pageState.handleCountChange} />
		{/if}
	</div>

	<Buttons>
		<Button
			onClick={pageState.handleConfirm}
			disabled={pageState.saving || !mapRowState.pointA || !mapRowState.pointB}
			nowrap
		>
			{locale.addRowConfirmButton(mapRowState.count)}
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
			display: flex;
			flex-direction: column;
			gap: var(--gap);

			.line {
				display: flex;
				flex-direction: row;
				align-items: center;
				gap: var(--gap);

				.icon {
					width: 24px;
					height: 24px;
					flex-shrink: 0;
				}

				.value {
					flex-grow: 1;
					font-size: 0.9rem;
					opacity: 0.8;
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
