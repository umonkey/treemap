<script lang="ts">
	import { locale } from '$lib/locale';
	import { NarrowPage } from '$lib/ui';
	import { routes } from '$lib/routes';
	import ChangeHistory from '$lib/components/tree/ChangeHistory.svelte';
	import TreeForm from '$lib/components/forms/TreeForm.svelte';
	import { CircumferenceInput } from '$lib/ui';
	import { girthState } from './hooks.svelte.ts';

	const { data } = $props();

	$effect(() => {
		girthState.reload(data.id);
	});
</script>

<NarrowPage title={locale.measureTitle()} back={routes.mapPreview(data.id)}>
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
</NarrowPage>
