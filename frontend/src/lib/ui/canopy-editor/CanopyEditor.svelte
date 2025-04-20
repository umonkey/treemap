<script lang="ts">
	import type { ITree } from '$lib/types';
	import { Button, Buttons } from '$lib/ui';
	import { HelpIcon } from '$lib/icons';
	import { locale } from '$lib/locale';
	import { updateCrownDiameter } from '$lib/actions';

	const { tree } = $props<{
		tree: ITree;
	}>();

	const { busy, handleConfirm, handleCancel } = updateCrownDiameter(tree.id);

	let value = $state<number>(tree.diameter ?? 0);

	const onSave = async () => {
		handleConfirm(value);
	};
</script>

<form class="form" onsubmit={onSave}>
	<label for="control">{locale.measureCanopy()}</label>
	<div class="row">
		<!-- svelte-ignore a11y_autofocus -->
		<input id="control" type="number" step="0.1" bind:value autofocus />
		<a class="icon" href="https://myga.am/app/measuring-canopy.html" target="_blank"><HelpIcon /></a
		>
	</div>
	<Buttons>
		<Button label={locale.editSave()} type="submit" onClick={onSave} disabled={$busy} />
		<Button label={locale.editCancel()} type="cancel" onClick={handleCancel} />
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
