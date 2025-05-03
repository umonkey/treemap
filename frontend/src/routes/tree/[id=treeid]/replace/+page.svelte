<script lang="ts">
	import { locale } from '$lib/locale';
	import { ReplaceForm } from '$lib/forms';
	import { NarrowPage } from '$lib/ui';

	const { data } = $props();
</script>

<NarrowPage title={locale.replaceTitle()}>
	<ReplaceForm id={data.id} />
</NarrowPage>
