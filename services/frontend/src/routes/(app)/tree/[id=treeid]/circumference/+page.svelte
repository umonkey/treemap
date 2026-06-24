<script lang="ts">
	import { page } from '$app/stores';
	import TreeForm from '$lib/components/forms/TreeForm.svelte';
	import CircumferenceInput from '$lib/ui/circumference-input/CircumferenceInput.svelte';
	import ChangeHistory from '../components/ChangeHistory.svelte';
	import { girthState } from './hooks.svelte.ts';

	const id = $derived($page.params.id as string);

	$effect(() => {
		girthState.reload(id);
	});
</script>

<TreeForm
	{id}
	title="Tree Circumference"
	onSubmit={girthState.save}
	onCancel={girthState.close}
	canSave={girthState.canSave}
	saving={girthState.saving}
>
	<CircumferenceInput value={null} autofocus onChange={girthState.handleChange} />
	<ChangeHistory {id} name="circumference" />
</TreeForm>
