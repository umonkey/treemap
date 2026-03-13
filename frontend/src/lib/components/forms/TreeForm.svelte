<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { Tree } from '$lib/types';
	import { locale } from '$lib/locale';
	import Title from '$lib/components/tree/Title.svelte';
	import TreeContextMenu from '$lib/components/tree/TreeContextMenu.svelte';
	import Button from '$lib/ui/button/Button.svelte';
	import Buttons from '$lib/ui/buttons/Buttons.svelte';

	type Props = {
		children?: Snippet;
		tree: Tree;
		saving: boolean;
		onSubmit: () => {};
		onCancel: () => {};
	};

	const { children, tree, saving = false, onSubmit, onCancel }: Props = $props();

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
</script>

<form onsubmit={handleSubmit} onkeydown={handleKeyDown}>
	<div class="buttons phone">
		<Button type="cancel" onClick={handleCancel} disabled={saving}>Cancel</Button>
		<div class="sep"></div>
		<Button type="submit" onClick={handleSubmit} disabled={saving}>Save</Button>
	</div>

	<Title title={tree.species} address={tree.address ?? undefined} />
	<TreeContextMenu id={tree.id} />

	{#if children}
		{@render children()}
	{/if}

	<div class="buttons desktop">
		<Button type="submit" onClick={handleSubmit} disabled={saving}>{locale.editSave()}</Button>
		<Button type="cancel" onClick={handleCancel} disabled={saving}>{locale.editCancel()}</Button>
	</div>
</form>

<style>
	form {
		position: relative;
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.buttons {
		display: flex;
		flex-direction: row;
		gap: 1rem;

		&.phone {
			display: none;
		}

		.sep {
			display: none;
		}
	}

	/** Sticky buttons on phones **/
	@media screen and (max-width: 600px) {
		form {
			padding-top: 40px;
		}

		.buttons {
			position: fixed;
			top: 40px;
			left: 0;
			width: 100%;
			box-sizing: border-box;

			z-index: 5;

			padding: 0.5rem;
			background-color: var(--background-color);

			&.desktop {
				display: none;
			}

			&.phone {
				display: flex;
			}

			.sep {
				display: block;
				flex: 1 1 auto;
			}
		}
	}
</style>
