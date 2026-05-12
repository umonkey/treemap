<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { locale } from '$lib/locale';
	import { pageState } from './page.svelte';
	import Button from '$lib/ui/button/Button.svelte';
	import Buttons from '$lib/ui/buttons/Buttons.svelte';
	import { formatSpecies, formatState, shortDetails } from '$lib/utils/trees';
	import LocationIcon from '$lib/icons/LocationIcon.svelte';
	import TagIcon from '$lib/icons/TagIcon.svelte';
	import BatteryIcon from '$lib/icons/BatteryIcon.svelte';
	import CloseIcon from '$lib/icons/CloseIcon.svelte';
	import '$lib/styles/variables.css';

	const id = $derived($page.params.id as string);

	onMount(() => {
		pageState.init(id);
		return pageState.destroy;
	});
</script>

<div class="panel">
	{#if pageState.tree}
		<div class="header">
			<div class="title">
				{formatSpecies(pageState.tree.species)}
			</div>
			<button class="close" onclick={() => pageState.handleCancel(id)}><CloseIcon /></button>
		</div>

		<div class="props">
			{#if pageState.tree.address}
				<div class="line">
					<div class="icon">
						<LocationIcon />
					</div>
					<div class="value">{pageState.tree.address}</div>
				</div>
			{/if}
			<div class="line">
				<div class="icon">
					<TagIcon />
				</div>
				<div class="value">{shortDetails(pageState.tree)}</div>
			</div>
			<div class="line">
				<div class="icon">
					<BatteryIcon />
				</div>
				<div class="value">{formatState(pageState.tree.state)}</div>
			</div>
		</div>

		{#if pageState.error}
			<p class="error">{pageState.error}</p>
		{/if}

		<Buttons>
			<Button onClick={() => pageState.handleConfirm(id)} disabled={pageState.busy} nowrap
				>{locale.contextMove()}</Button
			>
			<Button
				type="secondary"
				onClick={() => pageState.handleCancel(id)}
				disabled={pageState.busy}
				nowrap>{locale.editCancel()}</Button
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
