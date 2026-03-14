<script lang="ts">
	import ChangeHistory from '$lib/components/tree/ChangeHistory.svelte';
	import TreeForm from '$lib/components/forms/TreeForm.svelte';
	import { HeightInput } from '$lib/ui';
	import { NarrowPage } from '$lib/ui';
	import { locale } from '$lib/locale';
	import { routes } from '$lib/routes';
	import { heightState } from './hooks.svelte.ts';

	const { data } = $props();

	$effect(() => {
		heightState.reload(data.id);
	});
</script>

<NarrowPage title={locale.measureTitle()} back={routes.mapPreview(data.id)}>
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
</NarrowPage>
