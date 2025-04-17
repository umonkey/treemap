<script lang="ts">
import { apiClient } from "$lib/api";
import Button from "$lib/components/forms/Button.svelte";
import HelpIcon from "$lib/icons/HelpIcon.svelte";
import { locale } from "$lib/locale";
import { addTrees } from "$lib/stores/treeStore";
import { toast } from "@zerodevx/svelte-toast";

const { tree, onClose } = $props();

const value = $state<number>(Math.round((tree.circumference ?? 0) * 100));

const onSave = async () => {
	const meters = value / 100;
	const res = await apiClient.updateTreeCircumference(tree.id, meters);

	if (res.status >= 200 && res.status < 400) {
		addTrees([
			{
				...tree,
				circumference: meters,
			},
		]);

		toast.push(locale.measureTrunkUpdated());

		onClose();
	} else {
		toast.push("Error saving changes.");
	}
};
</script>

<form class="form" onsubmit={onSave}>
	<label for="control">{locale.measureTrunk()}</label>
	<div class="row">
		<!-- svelte-ignore a11y_autofocus -->
		<input id="control" type="number" bind:value autofocus />
		<a class="icon" href="https://myga.am/app/measuring-circumference.html" target="_blank"
			><HelpIcon /></a
		>
	</div>
	<div class="actions">
		<Button label={locale.editSave()} type="submit" onClick={onSave} />
		<Button label={locale.editCancel()} type="cancel" onClick={onClose} />
	</div>
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
