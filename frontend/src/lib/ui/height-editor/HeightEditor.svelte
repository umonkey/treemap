<script lang="ts">
	import { apiClient } from '$lib/api';
	import { Button, Buttons } from '$lib/ui';
	import { HelpIcon } from '$lib/icons';
	import { locale } from '$lib/locale';
	import { addTrees } from '$lib/stores/treeStore';
	import { toast } from '@zerodevx/svelte-toast';
	import type { ITree } from '$lib/types';

	const { tree, onClose } = $props<{
		tree: ITree;
		onClose: () => void;
	}>();

	let value = $state<number>(tree.height ?? 0);

	const onSave = async () => {
		const res = await apiClient.updateTreeHeight(tree.id, value);

		if (res.status >= 200 && res.status < 400) {
			addTrees([
				{
					...tree,
					height: value
				}
			]);

			toast.push(locale.measureHeightUpdated());

			onClose();
		} else {
			toast.push('Error saving changes.');
		}
	};
</script>

<form class="form" onsubmit={onSave}>
	<label for="control">{locale.measureHeight()}</label>
	<div class="row">
		<!-- svelte-ignore a11y_autofocus -->
		<input id="control" type="number" step="0.1" bind:value autofocus />
		<a class="icon" href="https://myga.am/app/measuring-height.html" target="_blank"><HelpIcon /></a
		>
	</div>

	<Buttons>
		<Button label={locale.editSave()} type="submit" onClick={onSave} />
		<Button label={locale.editCancel()} type="cancel" onClick={onClose} />
	</Buttons>
</form>

<style>
	.form {
		display: flex;
		flex-direction: column;
		gap: var(--gap);

		.row {
			display: flex;
			flex-direction: row;
			gap: var(--gap);
			align-items: center;
		}

		label {
			display: block;
			padding: 0;
			margin: 0;
			line-height: 38px;
		}

		:global(svg) {
			width: 30px;
			height: 30px;
			display: block;
		}
	}

	a.icon {
		color: inherit;
	}
</style>
