<script lang="ts">
	import TreeContextMenu from '$lib/components/tree/TreeContextMenu.svelte';
	import { locale } from '$lib/locale';
	import { menuState } from '$lib/stores/treeMenu';
	import AuthWrapper from '$lib/ui/auth-wrapper/AuthWrapper.svelte';
	import Button from '$lib/ui/button/Button.svelte';
	import type { Snippet } from 'svelte';

	type Props = {
		id?: string;
		children?: Snippet;
		title?: string;
		saving?: boolean;
		onSubmit: () => void;
		onCancel: () => void;
		tree?: unknown;
	};

	const {
		id = undefined,
		children,
		title = 'Edit Tree',
		saving = false,
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

<AuthWrapper>
	<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
	<form onsubmit={handleSubmit} onkeydown={handleKeyDown}>
		<h2>{title}</h2>

		<div class="buttons phone">
			<button type="button" onclick={handleCancel} disabled={saving}>Cancel</button>
			<!-- svelte-ignore a11y_no_static_element_interactions -->
			<div class="sep" oncontextmenu={handleLongTap}>{title}</div>
			<button type="submit" onclick={handleSubmit} disabled={saving}>Save</button>
		</div>

		{#if children}
			{@render children()}
		{/if}

		<div class="buttons desktop">
			<Button type="submit" onClick={handleSubmit} disabled={saving}>{locale.editSave()}</Button>
			<Button type="cancel" onClick={handleCancel} disabled={saving}>{locale.editCancel()}</Button>
		</div>

		{#if id}
			<TreeContextMenu {id} />
		{/if}
	</form>
</AuthWrapper>

<style>
	form {
		position: relative;
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	h2 {
		font-size: 1.5rem;
		font-weight: 300;
		border-bottom: solid 1px rgba(128, 128, 128, 0.2);
		padding-bottom: 0.5rem;
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
