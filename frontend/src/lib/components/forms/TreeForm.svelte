<script lang="ts">
	import TreeContextMenu from '$lib/components/tree/TreeContextMenu.svelte';
	import { locale } from '$lib/locale';
	import { menuState } from '$lib/stores/treeMenu';
	import AuthWrapper from '$lib/ui/auth-wrapper/AuthWrapper.svelte';
	import Button from '$lib/ui/button/Button.svelte';
	import Overlay from '$lib/components/layout/Overlay.svelte';
	import type { Snippet } from 'svelte';

	type Props = {
		id?: string;
		children?: Snippet;
		title?: string;
		saving?: boolean;
		canSave?: boolean;
		onSubmit: () => void;
		onCancel: () => void;
		tree?: unknown;
	};

	const {
		id = undefined,
		children,
		title = 'Edit Tree',
		saving = false,
		canSave = true,
		onSubmit,
		onCancel
	}: Props = $props();

	const handleSubmit = (e?: Event) => {
		e?.preventDefault();
		onSubmit();
	};

	const handleCancel = (e?: Event) => {
		e?.preventDefault();
		onCancel();
	};

	const handleKeyDown = (event: KeyboardEvent) => {
		if (event.key === 'Enter' && event.ctrlKey) {
			onSubmit();
		}
	};

	const handleLongTap = (e: Event) => {
		e.preventDefault();
		menuState.update((value) => !value);
	};
</script>

<Overlay onClick={handleCancel}>
	<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
	<form class="dialog" onsubmit={handleSubmit} onkeydown={handleKeyDown}>
		<div class="title">
			<button type="button" onclick={handleCancel} disabled={saving}>Cancel</button>
			<!-- svelte-ignore a11y_no_static_element_interactions -->
			<h1 oncontextmenu={handleLongTap}>{title}</h1>
			<button type="submit" disabled={!canSave || saving}>Save</button>
		</div>

		<div class="body">
			{#if children}
				<AuthWrapper>
					{@render children()}
				</AuthWrapper>
			{/if}
		</div>

		{#if id}
			<TreeContextMenu {id} />
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
		max-width: 600px;
		width: 600px;
		transform: translate(-50%, -50%);

		background-color: var(--background-color);
		border-radius: 10px;
		overflow: hidden;
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
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

		button {
			border: none;
			background-color: inherit;
			flex: 0 0 50px;
			opacity: 0.75;
			font-size: 0.9rem;
			cursor: pointer;
		}
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

	h2 {
		font-size: 1.5rem;
		font-weight: 300;
		border-bottom: solid 1px rgba(128, 128, 128, 0.2);
		padding-bottom: 0.5rem;
		margin: 1rem 0 0;
	}

	.buttons {
		display: flex;
		flex-direction: row;
		gap: 1rem;
		align-items: center;
	}

	.buttons.phone {
		background-color: light-dark(#bfc7d9, #333c4e);
		position: fixed;
		top: 0px;
		left: 0;
		width: 100%;
		box-sizing: border-box;
		z-index: 5;
		padding: 0 0.5rem;
		line-height: 40px;

		.sep {
			flex: 1 1 auto;
			text-align: center;
			white-space: nowrap;
			overflow: hidden;
			text-overflow: ellipsis;

			/** Prevent text selection on context menu **/
			user-select: none;
			-webkit-user-select: none;
			-webkit-touch-callout: none;
		}

		button {
			border: none;
			background-color: inherit;
			flex: 0 0 50px;
			opacity: 0.75;
			font-size: 0.9rem;
		}
	}

	/** Sticky buttons on phones **/
	@media screen and (max-width: 600px) {
		form {
			padding-top: 1rem;
		}

		h2 {
			display: none;
		}

		.buttons.desktop {
			display: none;
		}
	}

	/** Sticky buttons on phones **/
	@media screen and (min-width: 601px) {
		.buttons.phone {
			display: none;
		}
	}
</style>
