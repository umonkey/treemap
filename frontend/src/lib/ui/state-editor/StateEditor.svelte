<script lang="ts">
	import { apiClient } from '$lib/api';
	import { Button, StateInput } from '$lib/ui';
	import { locale } from '$lib/locale';
	import { addTrees } from '$lib/stores/treeStore';
	import { toast } from '@zerodevx/svelte-toast';
	import type { ITree } from '$lib/types';

	const { tree, onClose } = $props<{
		tree: ITree;
		onClose: () => void;
	}>();

	let value = $state<string>(tree.state ?? 'unknown');

	const onSave = async () => {
		const res = await apiClient.updateTreeState(tree.id, value);

		if (res.status >= 200 && res.status < 400) {
			addTrees([
				{
					...tree,
					state: value
				}
			]);

			toast.push(locale.measureStateUpdated());

			onClose();
		} else {
			toast.push('Error saving changes.');
		}
	};
</script>

<div class="form">
	<label for="control">{locale.measureState()}</label>
	<div class="row">
		<StateInput {value} label={false} onChange={(v: string) => (value = v)} />
	</div>
	<div class="actions">
		<Button label={locale.editSave()} type="submit" onClick={onSave} />
		<Button label={locale.editCancel()} type="cancel" onClick={onClose} />
	</div>
</div>

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
</style>
