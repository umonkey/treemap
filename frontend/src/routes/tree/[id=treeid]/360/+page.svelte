<script lang="ts">
	import { locale } from '$lib/locale';
	import { StreetViewPage, NarrowPage } from '$lib/ui';
	import { routes } from '$lib/routes';

	const { data } = $props();
</script>

<NarrowPage title={locale.treeShortTitle()} back={routes.mapPreview(data.id)} nopadding>
	<StreetViewPage id={data.id} />
</NarrowPage>
