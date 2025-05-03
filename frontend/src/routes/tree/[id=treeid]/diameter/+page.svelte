<script lang="ts">
	import { locale } from '$lib/locale';
	import { CrownForm } from '$lib/forms';
	import { NarrowPage } from '$lib/ui';

	const { data } = $props();
</script>

<NarrowPage title={locale.measureTitle()}>
	<CrownForm id={data.id} />
</NarrowPage>
