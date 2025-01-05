<script lang="ts">
	import Button from '$lib/components/forms/Button.svelte';
	import { apiClient } from '$lib/api';
	import { toast } from '@zerodevx/svelte-toast';
	import { addTrees } from '$lib/stores/treeStore';
	import { locale } from '$lib/locale';
	import StateInput from '$lib/components/forms/StateInput.svelte';

	const { tree, onClose } = $props();

	let value = $state<string | null>(tree.state);

	const onSave = async () => {
		const res = await apiClient.updateTreeState(tree.id, value);

		if (res.status >= 200 && res.status < 400) {
			addTrees([
				{
					...tree,
					state: value
				}
			]);

			toast.push('Tree state updated.');

			onClose();
		} else {
			toast.push('Error saving changes.');
		}
	};
</script>

<div class="form">
	<label for="control">{locale.measureState()}</label>
	<div class="row">
		<StateInput bind:value label={false} />
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
	}

	:global(svg) {
		width: 30px;
		height: 30px;
		display: block;
	}
</style>
