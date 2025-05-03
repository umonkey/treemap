<script lang="ts">
	import { locale } from '$lib/locale';
	import { HeightForm } from '$lib/forms';
	import { NarrowPage } from '$lib/ui';

	const { data } = $props();
</script>

<NarrowPage title={locale.measureTitle()}>
	<HeightForm id={data.id} />
</NarrowPage>
