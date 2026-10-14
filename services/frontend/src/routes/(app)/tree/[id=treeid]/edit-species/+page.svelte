<script lang="ts">
	import { page } from '$app/state';
	import TreeForm from '$lib/components/forms/TreeForm.svelte';
	import SpeciesInput from '$lib/ui/species-input/SpeciesInput.svelte';
	import ChangeHistory from '../components/ChangeHistory.svelte';
	import { SpeciesState } from './hooks.svelte.ts';

	const speciesState = new SpeciesState();

	const id = $derived(page.params.id as string);

	$effect(() => {
		speciesState.reload(id);
	});
</script>

<TreeForm
	onSubmit={speciesState.save}
	onCancel={speciesState.close}
	canSave={speciesState.canSave}
	saving={speciesState.saving}
>
	<SpeciesInput value={null} onChange={speciesState.handleChange} />
	<ChangeHistory {id} name="species" />
</TreeForm>
