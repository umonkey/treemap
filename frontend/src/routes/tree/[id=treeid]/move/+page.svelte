<script lang="ts">
	import { locale } from '$lib/locale';
	import MoveForm from './MoveForm.svelte';
	import { NarrowPage } from '$lib/ui';
	import { routes } from '$lib/routes';

	const { data } = $props();
</script>

<NarrowPage title={locale.measureTitle()} back={routes.mapPreview(data.id)}>
	<MoveForm id={data.id} />
</NarrowPage>
