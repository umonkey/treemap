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
		<Button label={locale.editSave()} type="submit" onClick={save} />
		<Button label={locale.editCancel()} type="cancel" onClick={close} />
	</Buttons>
</Form>
