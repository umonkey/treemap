<script lang="ts">
	import ChangeHistory from '$lib/components/tree/ChangeHistory.svelte';
	import TreeForm from '$lib/components/forms/TreeForm.svelte';
	import { HeightInput } from '$lib/ui';
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import { locale } from '$lib/locale';
	import { heightState } from './hooks.svelte.ts';

	const { data } = $props();

	$effect(() => {
		heightState.reload(data.id);
	});
</script>

<Dialog title={locale.measureTitle()}>
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
</Dialog>
