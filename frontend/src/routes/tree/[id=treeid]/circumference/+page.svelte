<script lang="ts">
	import { locale } from '$lib/locale';
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import ChangeHistory from '$lib/components/tree/ChangeHistory.svelte';
	import TreeForm from '$lib/components/forms/TreeForm.svelte';
	import { CircumferenceInput } from '$lib/ui';
	import { girthState } from './hooks.svelte.ts';

	const { data } = $props();

	$effect(() => {
		girthState.reload(data.id);
	});
</script>

<Dialog title={locale.measureTitle()}>
	<TreeForm
		id={data.id}
		title="Tree Circumference"
		onSubmit={girthState.save}
		onCancel={girthState.close}
		canSave={girthState.canSave}
	>
		<CircumferenceInput value={null} autofocus onChange={girthState.handleChange} />
		<ChangeHistory id={data.id} name="circumference" />
	</TreeForm>
</Dialog>
