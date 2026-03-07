<script lang="ts">
	import { locale } from '$lib/locale';
	import { CircumferenceForm } from '$lib/forms';
	import { NarrowPage } from '$lib/ui';
	import { routes } from '$lib/routes';

	const { data } = $props();
</script>

<NarrowPage title={locale.measureTitle()} back={routes.mapPreview(data.id)}>
	<CircumferenceForm id={data.id} />
</NarrowPage>
