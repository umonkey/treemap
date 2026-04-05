<script lang="ts">
	import TreeForm from '$lib/components/forms/TreeForm.svelte';
	import CanopyInput from '$lib/ui/canopy-input/CanopyInput.svelte';
	import ChangeHistory from '../components/ChangeHistory.svelte';
	import { crownState } from './hooks.svelte.ts';

	const { data } = $props();

	$effect(() => {
		crownState.reload(data.id);
	});
</script>

<TreeForm
	id={data.id}
	title="Crown Diameter"
	onSubmit={crownState.save}
	onCancel={crownState.close}
	canSave={crownState.canSave}
	saving={crownState.saving}
>
	<CanopyInput value={null} autofocus onChange={crownState.handleChange} />
	<ChangeHistory id={data.id} name="diameter" />
</TreeForm>
