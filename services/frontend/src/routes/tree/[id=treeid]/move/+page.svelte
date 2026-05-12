<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { locale } from '$lib/locale';
	import { mapMode } from '$lib/stores/mapMode';
	import { getTree, updateTreeLocation } from '$lib/api/trees';
	import { mapBus } from '$lib/buses/mapBus';
	import { mapState } from '$lib/components/map/MapLibre.svelte.ts';
	import Button from '$lib/ui/button/Button.svelte';
	import Buttons from '$lib/ui/buttons/Buttons.svelte';
	import { goto, routes } from '$lib/routes';
	import { roundCoord } from '$lib/utils/strings';
	import type { ITree } from '$lib/types';
	import { formatSpecies, formatState, shortDetails } from '$lib/utils/trees';
	import LocationIcon from '$lib/icons/LocationIcon.svelte';
	import TagIcon from '$lib/icons/TagIcon.svelte';
	import BatteryIcon from '$lib/icons/BatteryIcon.svelte';
	import CloseIcon from '$lib/icons/CloseIcon.svelte';
	import '$lib/styles/variables.css';

	const id = $derived($page.params.id as string);
	let tree = $state<ITree | undefined>(undefined);
	let busy = $state(false);
	let error = $state<string | undefined>(undefined);

	onMount(() => {
		mapMode.set('move');

		getTree(id).then((res) => {
			if (res.data) {
				tree = res.data;
				mapBus.emit('move', { lat: res.data.lat, lng: res.data.lon });
			}
		});

		return () => {
			mapMode.set(undefined);
		};
	});

	async function handleConfirm() {
		busy = true;
		error = undefined;

		const lat = roundCoord(mapState.center.lat);
		const lng = roundCoord(mapState.center.lng);

		const res = await updateTreeLocation(id, lat, lng);
		if (res.status >= 200 && res.status < 300) {
			goto(routes.mapPreview(id));
		} else {
			error = res.error?.description || 'Failed to move tree';
			busy = false;
		}
	}

	function handleCancel() {
		goto(routes.home());
	}
</script>

<div class="panel">
	{#if tree}
		<div class="header">
			<div class="title">
				{formatSpecies(tree.species)}
			</div>
			<button class="close" onclick={handleCancel}><CloseIcon /></button>
		</div>

		<div class="props">
			{#if tree.address}
				<div class="line">
					<div class="icon">
						<LocationIcon />
					</div>
					<div class="value">{tree.address}</div>
				</div>
			{/if}
			<div class="line">
				<div class="icon">
					<TagIcon />
				</div>
				<div class="value">{shortDetails(tree)}</div>
			</div>
			<div class="line">
				<div class="icon">
					<BatteryIcon />
				</div>
				<div class="value">{formatState(tree.state)}</div>
			</div>
		</div>

		{#if error}
			<p class="error">{error}</p>
		{/if}

		<Buttons>
			<Button onClick={handleConfirm} disabled={busy} nowrap>{locale.contextMove()}</Button>
			<Button type="secondary" onClick={handleCancel} disabled={busy} nowrap
				>{locale.editCancel()}</Button
			>
		</Buttons>
	{/if}
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

		.error {
			color: var(--color-error);
			margin: 0;
			font-size: 0.9rem;
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
