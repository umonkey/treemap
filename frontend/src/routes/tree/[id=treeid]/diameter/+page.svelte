<script lang="ts">
	import { locale } from '$lib/locale';
	import { CrownForm } from '$lib/forms';
	import { NarrowPage } from '$lib/ui';
	import { routes } from '$lib/routes';

	const { data } = $props();
</script>

<NarrowPage title={locale.measureTitle()} back={routes.mapPreview(data.id)}>
	<CrownForm id={data.id} />
</NarrowPage>
