<script lang="ts">
	import HelpIcon from '$lib/icons/HelpIcon.svelte';
	import Button from '$lib/components/forms/Button.svelte';
	import { apiClient } from '$lib/api';
	import { toast } from '@zerodevx/svelte-toast';
	import { addTrees } from '$lib/stores/treeStore';
	import { locale } from '$lib/locale';

	const { tree, onClose } = $props();

	let value = $state<number>(tree.diameter ?? 0);

	const onSave = async () => {
		const res = await apiClient.updateTreeDiameter(tree.id, value);

		if (res.status >= 200 && res.status < 400) {
			addTrees([
				{
					...tree,
					diameter: value
				}
			]);

			toast.push('Canopy diameter updated.');

			onClose();
		} else {
			toast.push('Error saving changes.');
		}
	};
</script>

<div class="form">
	<label for="control">{locale.measureCanopy()}</label>
	<div class="row">
		<!-- svelte-ignore a11y_autofocus -->
		<input id="control" type="number" bind:value autofocus />
		<a class="icon" href="https://myga.am/app/measuring-canopy.html" target="_blank"><HelpIcon /></a
		>
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

	a.icon {
		color: inherit;
	}
</style>
