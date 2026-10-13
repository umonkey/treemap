<script lang="ts">
	import { page } from '$app/stores';
	import TreeForm from '$lib/components/forms/TreeForm.svelte';
	import CircumferenceInput from '$lib/ui/circumference-input/CircumferenceInput.svelte';
	import ChangeHistory from '../components/ChangeHistory.svelte';
	import { GirthState } from './hooks.svelte.ts';

	const girthState = new GirthState();

	const id = $derived($page.params.id as string);

	$effect(() => {
		girthState.reload(id);
	});
</script>

<TreeForm
	onSubmit={girthState.save}
	onCancel={girthState.close}
	canSave={girthState.canSave}
	saving={girthState.saving}
>
	<CircumferenceInput value={null} autofocus onChange={girthState.handleChange} />
	<ChangeHistory {id} name="circumference" />
</TreeForm>
