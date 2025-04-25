<script lang="ts">
	import type { ITree } from '$lib/types';
	import { Button, StateInput } from '$lib/ui';
	import { editor } from './hooks';
	import { locale } from '$lib/locale';

	const { tree } = $props<{
		tree: ITree;
	}>();

	const { value, save, handleChange, close } = editor(tree);
</script>

<div class="form">
	<label for="control">{locale.measureState()}</label>
	<div class="row">
		<StateInput {value} label={false} onChange={handleChange} />
	</div>
	<div class="actions">
		<Button label={locale.editSave()} type="submit" onClick={save} />
		<Button label={locale.editCancel()} type="cancel" onClick={close} />
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
