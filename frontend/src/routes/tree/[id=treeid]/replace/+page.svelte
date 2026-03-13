<script lang="ts">
	import { locale } from '$lib/locale';
	import ReplaceForm from './ReplaceForm.svelte';
	import { NarrowPage } from '$lib/ui';
	import { routes } from '$lib/routes';

	const { data } = $props();
</script>

<NarrowPage title={locale.replaceTitle()} back={routes.mapPreview(data.id)}>
	<ReplaceForm id={data.id} />
</NarrowPage>
