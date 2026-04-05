<script lang="ts">
	import { page } from '$app/stores';
	import TreeForm from '$lib/components/forms/TreeForm.svelte';
	import CanopyInput from '$lib/ui/canopy-input/CanopyInput.svelte';
	import ChangeHistory from '../components/ChangeHistory.svelte';
	import { crownState } from './hooks.svelte.ts';

	const id = $derived($page.params.id as string);

	$effect(() => {
		crownState.reload(id);
	});
</script>

<TreeForm
	{id}
	title="Crown Diameter"
	onSubmit={crownState.save}
	onCancel={crownState.close}
	canSave={crownState.canSave}
	saving={crownState.saving}
>
	<CanopyInput value={null} autofocus onChange={crownState.handleChange} />
	<ChangeHistory {id} name="diameter" />
</TreeForm>
