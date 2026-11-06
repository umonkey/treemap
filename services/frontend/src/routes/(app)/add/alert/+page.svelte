<script lang="ts">
	import { locale as appLocale } from '$lib/locale';
	import { PageState } from './page.svelte';
	import { locale } from './lang';
	import Button from '$lib/ui/button/Button.svelte';
	import TextInput from '$lib/ui/text-input/TextInput.svelte';
	import CloseIcon from '$lib/icons/CloseIcon.svelte';
	import PhotoPicker from './PhotoPicker.svelte';
	import PhotoQueue from './PhotoQueue.svelte';
	import { mapMode } from '$lib/stores/mapMode';
	import CloseOnEscape from '$lib/components/layout/CloseOnEscape.svelte';
	import { mapBottomPadding } from '$lib/components/map/mapBottomPadding';
	import { onMount } from 'svelte';
	import '$lib/styles/variables.css';

	const pageState = new PageState();

	onMount(() => {
		$mapMode = 'add';
		return () => {
			$mapMode = undefined;
			pageState.destroy();
		};
	});
</script>

<svelte:head>
	<title>{locale.title()} — {appLocale.appTitle()}</title>
</svelte:head>

<CloseOnEscape onClose={pageState.handleCancel} />

<div class="panel" use:mapBottomPadding>
	<div class="header">
		<div class="title">
			{locale.title()}
		</div>
		<button class="close" aria-label={appLocale.editCancel()} onclick={pageState.handleCancel}
			><CloseIcon /></button
		>
	</div>

	<TextInput
		ariaLabel={locale.descriptionAriaLabel()}
		placeholder={locale.descriptionPlaceholder()}
		value={pageState.description}
		multiline
		onChange={(value) => (pageState.description = value)}
	/>

	<PhotoQueue
		items={pageState.photos.map((photo) => ({ url: photo.url }))}
		onRemove={pageState.removePhoto}
	/>

	<div class="actions">
		<PhotoPicker onFiles={pageState.addFiles} disabled={pageState.saving} />

		<Button onClick={pageState.handleSend} disabled={!pageState.canSend} nowrap>
			{pageState.saving ? locale.sendingButton() : locale.sendButton()}
		</Button>
	</div>
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
		max-height: 80dvh;
		overflow-y: auto;
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

		.actions {
			display: flex;
			flex-direction: row;
			align-items: center;
			gap: 10px;
		}
	}

	/* Desktop */
	@media (min-width: 1024px) {
		.panel {
			top: 0;
			left: 0;
			width: var(--sidebar-width-desktop);
			height: 100vh;
			max-height: none;
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
			max-height: 70dvh;
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
