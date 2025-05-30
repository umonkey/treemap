<script lang="ts">
	import { locale } from '$lib/locale';
	import { StreetViewPage, NarrowPage } from '$lib/ui';

	const { data } = $props();
</script>

<NarrowPage title={locale.treeShortTitle()} nopadding>
	<StreetViewPage id={data.id} />
</NarrowPage>
