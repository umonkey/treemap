<script lang="ts">
	import ObservationForm from './ObservationForm.svelte';
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import { locale } from '$lib/locale';
	import { routes } from '$lib/routes';

	const { data } = $props();
	const tree = data.tree;
</script>

<Dialog title={locale.treeTabsObservations()}>
	<ObservationForm id={tree.id} />
</Dialog>
