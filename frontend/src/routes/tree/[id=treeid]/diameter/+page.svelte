<script lang="ts">
	import { locale } from '$lib/locale';
	import { NarrowPage } from '$lib/ui';
	import { routes } from '$lib/routes';
	import ChangeHistory from '$lib/components/tree/ChangeHistory.svelte';
	import TreeForm from '$lib/components/forms/TreeForm.svelte';
	import { CanopyInput } from '$lib/ui';
	import { crownState } from './hooks.svelte.ts';

	const { data } = $props();

	$effect(() => {
		crownState.reload(data.id);
	});
</script>

<NarrowPage title={locale.measureTitle()} back={routes.mapPreview(data.id)}>
	<TreeForm
		id={data.id}
		title="Crown Diameter"
		onSubmit={crownState.save}
		onCancel={crownState.close}
		canSave={crownState.canSave}
	>
		<CanopyInput value={null} autofocus onChange={crownState.handleChange} />
		<ChangeHistory id={data.id} name="diameter" />
	</TreeForm>
</NarrowPage>
