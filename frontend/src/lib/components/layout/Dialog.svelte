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

	const { title, children, buttons, header } = $props<{
		title: string;
		children: Snippet;
		buttons?: ButtonDef[];
		header?: Snippet;
	}>();
</script>

<svelte:head>
	<title>{title}</title>
</svelte:head>

<Overlay onClick={handleClose}>
	<form class="dialog">
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

		<div class="body">
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

		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);

		background-color: var(--background-color);
		border-radius: 10px;
		overflow: hidden;
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
	}

	.button {
		flex: 0 0 40px;
		display: block;
		height: 40px;
	}

	.title,
	.actions {
		width: 100%;
		padding: 0.5rem 1rem;
		box-sizing: border-box;

		height: 40px;
		background-color: rgba(0, 0, 0, 0.2);

		display: flex;
		flex-direction: row;
		align-items: center;
	}

	.title {
		padding: 0.5rem 0;
	}

	.actions {
		height: 60px;
	}

	h1 {
		font-size: 1.25rem;
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

	@media screen and (min-width: 1024px) {
		.dialog {
			width: 600px;
			max-width: 600px;
		}
	}

	/** Make the dialog full-screen on mobile devices. **/
	@media screen and (max-width: 600px) {
		.dialog {
			width: 100vw;
			height: calc(100vh - var(--bottom-nav-height));
			border-radius: 0;
			max-width: 100vw;

			transform: none;
			top: 0;
			left: 0;
			bottom: 0;
		}

		.body {
			max-height: none;
		}
	}
</style>
