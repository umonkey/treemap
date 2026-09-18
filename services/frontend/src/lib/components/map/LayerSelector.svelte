<script lang="ts">
	import Icon from '$lib/icons/Layers.svelte';
	import CheckInput from '$lib/ui/check-input/CheckInput.svelte';
	import SelectButton from '$lib/ui/SelectButton.svelte';
	import { Control } from 'svelte-maplibre';
	import { locale } from './LayerSelector.lang';
	import { LayerSelector } from './LayerSelector.svelte.ts';
	import { fade } from 'svelte/transition';

	const componentState = new LayerSelector();

	$effect(() => componentState.init());
</script>

<Control position="top-right">
	<div class="maplibregl-ctrl-group">
		<button class="button" type="button" title="Switch layers" onclick={componentState.toggle}>
			<Icon />
		</button>
	</div>
</Control>

{#if componentState.open}
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<div
		class="backdrop"
		onclick={componentState.handleBackdropClick}
		oncontextmenu={componentState.handleBackdropContextMenu}
	>
		<div class="menu" out:fade={{ duration: 100 }}>
			<div class="base-layers">
				<SelectButton
					value="light"
					label={locale.baseLight()}
					active={componentState.base === 'light'}
					onClick={componentState.setBase}
				/>
				<SelectButton
					value="basic"
					label={locale.baseDetails()}
					active={componentState.base === 'basic'}
					onClick={componentState.setBase}
				/>
				<SelectButton
					value="google"
					label={locale.baseSatellite()}
					active={componentState.base === 'google'}
					onClick={componentState.setBase}
				/>
			</div>

			<div class="sep">
				<div class="additional-layers">
					<CheckInput
						value={componentState.drone}
						label={locale.layerDrone()}
						onChange={componentState.toggleDrone}
					/>
					<CheckInput
						value={componentState.panoramas}
						label={locale.layerPanoramas()}
						onChange={componentState.togglePanoramas}
					/>
					<CheckInput
						value={componentState.treeHints}
						label={locale.layerTreeHints()}
						onChange={componentState.toggleTreeHints}
					/>
					<CheckInput
						value={componentState.water}
						label={locale.layerWater()}
						onChange={componentState.toggleWater}
					/>
					<CheckInput
						value={componentState.alerts}
						label={locale.layerAlerts()}
						onChange={componentState.toggleAlerts}
					/>
				</div>
			</div>

			<div class="sep">
				<div class="other-options">
					<CheckInput
						value={componentState.stickyPoints}
						label={locale.optionStickyPoints()}
						onChange={componentState.toggleStickyPoints}
					/>
					<CheckInput
						value={componentState.center}
						label={locale.optionCenter()}
						onChange={componentState.toggleCenter}
					/>
				</div>
			</div>
		</div>
	</div>
{/if}

<style>
	.button {
		padding: 4px;
		color: #000;
	}

	.backdrop {
		position: fixed;
		inset: 0;
		z-index: 12;
	}

	.menu {
		position: fixed;
		top: 2rem;
		right: 2rem;
		z-index: 12;

		width: 320px;
		max-width: calc(100vw - 2rem);
		max-height: calc(100vh - 2rem);
		overflow-y: auto;

		font-size: 1rem;

		padding: 0;
		background-color: var(--map-menu-background);
		border-radius: 10px;
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);

		display: flex;
		flex-direction: column;

		.base-layers {
			display: flex;
			gap: 0.5rem;
			width: 100%;
			padding: 1rem 1rem 0.5rem;
			box-sizing: border-box;

			:global(button) {
				flex: 1;
			}
		}

		.sep {
			border-top: 1px solid rgba(128, 128, 128, 0.2);
			padding: 0.5rem 20px;
			margin-top: 0.5rem;
			box-sizing: border-box;

			&:last-child {
				padding-bottom: 1rem;
			}
		}
	}

	.additional-layers,
	.other-options {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}
</style>
