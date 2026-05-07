<script lang="ts">
	import Overlay from '$lib/components/layout/Overlay.svelte';
	import CloseIcon from '$lib/icons/CloseIcon.svelte';
	import Button from '$lib/ui/button/Button.svelte';
	import type { Snippet } from 'svelte';
	import { handleClose } from './Dialog.svelte.ts';

	type ButtonDef = {
		title: string;
		onClick: () => void;
	};

	const {
		title,
		children,
		buttons,
		header,
		nopadding,
		variant = 'standard'
	} = $props<{
		title: string;
		children: Snippet;
		nopadding?: boolean;
		buttons?: ButtonDef[];
		header?: Snippet;
		variant?: 'standard' | 'bottom';
	}>();
</script>

<svelte:head>
	<title>{title}</title>
</svelte:head>

<Overlay onClick={handleClose}>
	<form class="dialog" class:variant-bottom={variant === 'bottom'}>
		{#if header}
			{@render header()}
		{:else}
			<div class="title">
				<div class="button"></div>
				<h1>{title}</h1>
				<div class="button">
					<button type="button" onclick={handleClose}>
						<CloseIcon />
					</button>
				</div>
			</div>
		{/if}

		<div class="body" class:nopadding={!!nopadding} class:nobuttons={!buttons}>
			{@render children()}
		</div>

		{#if buttons}
			<div class="actions">
				{#each buttons as button}
					<Button onClick={button.onClick}>{button.title}</Button>
				{/each}
			</div>
		{/if}
	</form>
</Overlay>

<style>
	.dialog {
		position: absolute;
		display: flex;
		flex-direction: column;

		width: 600px;
		max-width: 600px;
		margin-left: calc((100vw - 600px) / 2);

		top: 50%;
		transform: translateY(-50%);

		background-color: var(--color-dialog-background);
		border-radius: 10px;
		overflow: hidden;
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
	}

	.button {
		flex: 0 0 40px;
		display: block;
		height: 40px;

		:global(svg) {
			width: 22px;
			height: 22px;
		}
	}

	.title,
	.actions {
		width: 100%;
		padding: 0.5rem 1rem;
		box-sizing: border-box;

		background-color: var(--color-dialog-header);

		display: flex;
		flex-direction: row;
		align-items: center;
	}

	.title {
		padding: 0.5rem 0;
		height: var(--dialog-header-size);
	}

	.actions {
		height: var(--dialog-footer-size);
	}

	h1 {
		font-size: 1rem;
		font-weight: 400;
		line-height: 40px;
		text-align: center;
		margin: 0;

		flex: 1 0 auto;
	}

	.body {
		margin: 0 auto;
		padding: 1rem;
		width: 100%;
		box-sizing: border-box;

		min-height: 40vh;
		max-height: 80vh;
		overflow-x: hidden;
		overflow-y: scroll;

		display: flex;
		flex-direction: column;
		gap: 1rem;
		align-items: stretch;

		&.nopadding {
			padding: 1rem 0;
		}
	}

	button {
		background-color: transparent;
		border: none;
		border-radius: 5px;
		height: 40px;
		width: 40px;
		cursor: pointer;
		opacity: 0.5;
	}

	@media screen and (max-width: 600px) {
		.dialog {
			margin-left: 0;
		}
	}

	/** Make the dialog full-screen on mobile devices. **/
	@media screen and (max-width: 1023px) {
		.dialog {
			width: 100vw;
			height: calc(100dvh - var(--bottom-nav-height));
			border-radius: 0;

			transform: none;
			top: 0;
			left: 0;
			bottom: 0;

			&.variant-bottom {
				top: auto;
				bottom: 0;
				height: auto;
				max-height: 80dvh;
				border-top-left-radius: 8px;
				border-top-right-radius: 8px;
				background-color: var(--map-menu-background);
				box-shadow: 0 -4px 12px rgba(0, 0, 0, 0.1);

				.title {
					background-color: transparent;
					border-bottom: none;
					padding: 0 var(--gap);
					height: auto;
					min-height: 40px;

					.button:first-child {
						display: none;
					}

					.button:last-child {
						flex-basis: 30px;
						height: 30px;

						button {
							width: 30px;
							height: 30px;
						}
					}

					h1 {
						text-align: left;
						font-size: 120%;
						line-height: 30px;
						flex-shrink: 1;
						white-space: nowrap;
						overflow: hidden;
						text-overflow: ellipsis;
					}
				}

				.body {
					height: auto;
					max-height: none;
					min-height: auto;
					padding-bottom: calc(1rem + env(safe-area-inset-bottom));
				}

				animation: slideUp 0.2s ease-out;
			}
		}

		.body {
			min-height: auto;
			max-height: none;
			flex: 1;
			padding-bottom: calc(1rem + env(safe-area-inset-bottom));
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
