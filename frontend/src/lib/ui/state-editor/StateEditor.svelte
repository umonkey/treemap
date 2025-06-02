<script lang="ts">
	import type { ITree } from '$lib/types';
	import { Button, Buttons, Form, StateInput } from '$lib/ui';
	import { editor } from './hooks';
	import { locale } from '$lib/locale';

	const { tree } = $props<{
		tree: ITree;
	}>();

	const { value, save, handleChange, close } = editor(tree);
</script>

<Form>
	<label for="control">{locale.measureState()}</label>

	<div class="row">
		<StateInput {value} label={false} onChange={handleChange} />
	</div>

	<Buttons>
		<Button type="submit" onClick={save}>{locale.editSave()}</Button>
		<Button type="cancel" onClick={close}>{locale.editCancel()}</Button>
	</Buttons>
</Form>
