<script lang="ts">
	import { apiClient } from '$lib/api';
	import { Button, Buttons, Form } from '$lib/ui';
	import { HelpIcon } from '$lib/icons';
	import { locale } from '$lib/locale';
	import { addTrees } from '$lib/stores/treeStore';
	import { toast } from '@zerodevx/svelte-toast';
	import type { ITree } from '$lib/types';

	const { tree, onClose } = $props<{
		tree: ITree;
		onClose: () => void;
	}>();

	let value = $state<number>(Math.round((tree.circumference ?? 0) * 100));

	const onSave = async () => {
		const meters = value / 100;
		const res = await apiClient.updateTreeCircumference(tree.id, meters);

		if (res.status >= 200 && res.status < 400) {
			addTrees([
				{
					...tree,
					circumference: meters
				}
			]);

			toast.push(locale.measureTrunkUpdated());

			onClose();
		} else {
			toast.push('Error saving changes.');
		}
	};
</script>

<Form onSubmit={onSave}>
	<label for="control">{locale.measureTrunk()}</label>

	<div class="row">
		<!-- svelte-ignore a11y_autofocus -->
		<input id="control" type="number" bind:value autofocus />
		<a class="icon" href="https://myga.am/app/measuring-circumference.html" target="_blank"
			><HelpIcon /></a
		>
	</div>

	<Buttons>
		<Button label={locale.editSave()} type="submit" onClick={onSave} />
		<Button label={locale.editCancel()} type="cancel" onClick={onClose} />
	</Buttons>
</Form>
