<script lang="ts">
	import ObservationForm from './ObservationForm.svelte';
	import { NarrowPage } from '$lib/ui';
	import { locale } from '$lib/locale';
	import { routes } from '$lib/routes';

	const { data } = $props();
	const tree = data.tree;
</script>

<NarrowPage title={locale.treeTabsObservations()} back={routes.mapPreview(data.id)}>
	<ObservationForm id={tree.id} />
</NarrowPage>
