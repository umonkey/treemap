<script lang="ts">
	import { page } from '$app/state';
	import { locale as appLocale } from '$lib/locale';
	import { PageState } from './page.svelte';
	import { locale } from './lang';
	import Button from '$lib/ui/button/Button.svelte';
	import Form from '$lib/ui/form/Form.svelte';
	import TextInput from '$lib/ui/text-input/TextInput.svelte';
	import CloseIcon from '$lib/icons/CloseIcon.svelte';
	import SpinnerIcon from '$lib/icons/SpinnerIcon.svelte';
	import CloseOnEscape from '$lib/components/layout/CloseOnEscape.svelte';
	import { mapBottomPadding } from '$lib/components/map/mapBottomPadding';
	import '$lib/styles/variables.css';

	const pageState = new PageState();

	const id = $derived(page.params.id as string);

	$effect(() => {
		if (id) pageState.reload(id);
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

	{#if pageState.loading}
		<div class="loading-state">
			<SpinnerIcon />
		</div>
	{:else}
		<Form onSubmit={pageState.handleSave}>
			{#if pageState.error}
				<div class="error" role="alert">{pageState.error}</div>
			{/if}

			<TextInput
				ariaLabel={locale.descriptionAriaLabel()}
				placeholder={locale.descriptionPlaceholder()}
				value={pageState.description}
				multiline
				onChange={(value) => (pageState.description = value)}
			/>

			<div class="actions">
				<Button disabled={!pageState.canSave} nowrap>
					{pageState.saving ? locale.savingButton() : locale.saveButton()}
				</Button>
			</div>
		</Form>
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

		.loading-state {
			display: flex;
			align-items: center;
			justify-content: center;
			height: 100%;
			min-height: 100px;

			:global(svg) {
				width: 20px;
				height: 20px;
			}
		}

		.error {
			color: var(--color-learn-wrong-fg);
			background-color: var(--color-learn-wrong-bg);
			border-radius: 4px;
			padding: var(--gap);
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
