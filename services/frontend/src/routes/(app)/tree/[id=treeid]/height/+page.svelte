<script lang="ts">
	import { page } from '$app/stores';
	import TreeForm from '$lib/components/forms/TreeForm.svelte';
	import HeightInput from '$lib/ui/height-input/HeightInput.svelte';
	import ChangeHistory from '../components/ChangeHistory.svelte';
	import { heightState } from './hooks.svelte.ts';

	const id = $derived($page.params.id as string);

	$effect(() => {
		heightState.reload(id);
	});
</script>

<TreeForm
	{id}
	title="Tree Height"
	onSubmit={heightState.save}
	onCancel={heightState.close}
	canSave={heightState.canSave}
	saving={heightState.saving}
>
	<HeightInput value={null} autofocus onChange={heightState.handleChange} />
	<ChangeHistory {id} name="height" />
</TreeForm>
