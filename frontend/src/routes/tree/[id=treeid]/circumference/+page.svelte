<script lang="ts">
	import TreeForm from '$lib/components/forms/TreeForm.svelte';
	import CircumferenceInput from '$lib/ui/circumference-input/CircumferenceInput.svelte';
	import ChangeHistory from '../components/ChangeHistory.svelte';
	import { girthState } from './hooks.svelte.ts';

	const { data } = $props();

	$effect(() => {
		girthState.reload(data.id);
	});
</script>

<TreeForm
	id={data.id}
	title="Tree Circumference"
	onSubmit={girthState.save}
	onCancel={girthState.close}
	disabled={!girthState.canSave}
>
	<CircumferenceInput value={null} autofocus onChange={girthState.handleChange} />
	<ChangeHistory id={data.id} name="circumference" />
</TreeForm>
