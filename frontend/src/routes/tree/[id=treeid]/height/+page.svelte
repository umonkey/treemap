<script lang="ts">
	import TreeForm from '$lib/components/forms/TreeForm.svelte';
	import HeightInput from '$lib/ui/height-input/HeightInput.svelte';
	import ChangeHistory from '../components/ChangeHistory.svelte';
	import { heightState } from './hooks.svelte.ts';

	const { data } = $props();

	$effect(() => {
		heightState.reload(data.id);
	});
</script>

<TreeForm
	id={data.id}
	title="Tree Height"
	onSubmit={heightState.save}
	onCancel={heightState.close}
	canSave={heightState.canSave}
>
	<HeightInput value={null} autofocus onChange={heightState.handleChange} />
	<ChangeHistory id={data.id} name="height" />
</TreeForm>
