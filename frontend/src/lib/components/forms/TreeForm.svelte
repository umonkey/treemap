<script lang="ts">
	import { menuBus } from '$lib/buses/menuBus';
	import Overlay from '$lib/components/layout/Overlay.svelte';
	import AuthWrapper from '$lib/ui/auth-wrapper/AuthWrapper.svelte';
	import Button from '$lib/ui/button/Button.svelte';
	import type { Snippet } from 'svelte';

	type Props = {
		id?: string;
		children?: Snippet;
		title: string;
		canSave?: boolean;
		saving?: boolean;
		onSubmit: () => void;
		onCancel: () => void;
		tree?: unknown;
	};

	const {
		id = undefined,
		children,
		title = 'Edit Tree',
		canSave = true,
		saving = false,
		onSubmit,
		onCancel
	}: Props = $props();

	const handleSubmit = (e?: Event) => {
		if (!canSave || saving) {
			return;
		}
		e?.preventDefault();
		onSubmit();
	};

	const handleCancel = (e?: Event) => {
		if (saving) {
			return;
		}
		e?.preventDefault();
		onCancel();
	};

	const handleKeyDown = (event: KeyboardEvent) => {
		if (event.key === 'Enter' && event.ctrlKey) {
			handleSubmit();
		}
	};

	const handleLongTap = (e: Event) => {
		e.preventDefault();

		if (id) {
			menuBus.emit('show', id);
		}
	};
</script>

<Overlay onClick={handleCancel}>
	<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
	<form class="dialog" onsubmit={handleSubmit} onkeydown={handleKeyDown}>
		<div class="title">
			<button class="phone" type="button" onclick={handleCancel} disabled={saving}>Cancel</button>
			<h1 oncontextmenu={handleLongTap}>{title}</h1>
			<button class="phone" type="submit" disabled={!canSave || saving}>Save</button>
		</div>

		<div class="body">
			{#if children}
				<AuthWrapper>
					{@render children()}
				</AuthWrapper>
			{/if}
		</div>

		<div class="buttons desktop">
			<Button onClick={handleSubmit} disabled={!canSave || saving}>Save Changes</Button>
			<Button onClick={handleCancel} type="cancel" disabled={saving}>Cancel</Button>
		</div>
	</form>
</Overlay>

<style>
	.dialog {
		position: absolute;
		display: flex;
		flex-direction: column;

		top: 50%;
		left: 50%;
		max-width: 600px;
		width: 600px;
		transform: translate(-50%, -50%);

		background-color: var(--color-dialog-background);
		border-radius: 10px;
		overflow: hidden;
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
	}

	.title {
		width: 100%;
		padding: 0.5rem 1rem;
		box-sizing: border-box;

		height: 40px;
		background-color: var(--color-dialog-header);

		display: flex;
		flex-direction: row;
		align-items: center;
	}

	.title {
		padding: 0.5rem 0;

		button {
			border: none;
			background-color: inherit;
			height: 40px;
			padding: 0 1rem;
			flex: 0 0 50px;
			opacity: 0.75;
			font-size: 0.9rem;
			cursor: pointer;
		}
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
		max-width: 600px;
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

	.buttons {
		padding: 1rem;
		display: flex;
		flex-direction: row;
		gap: 1rem;
		background-color: var(--color-dialog-header);
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

		.desktop {
			display: none;
		}
	}

	@media screen and (min-width: 601px) {
		.phone {
			display: none;
		}
	}
</style>
