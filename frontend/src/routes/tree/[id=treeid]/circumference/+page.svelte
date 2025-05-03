<script lang="ts">
	import { locale } from '$lib/locale';
	import { CircumferenceForm } from '$lib/forms';
	import { NarrowPage } from '$lib/ui';

	const { data } = $props();
</script>

<NarrowPage title={locale.measureTitle()}>
	<CircumferenceForm id={data.id} />
</NarrowPage>
