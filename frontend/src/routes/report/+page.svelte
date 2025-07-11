<script lang="ts">
	import { locale } from '$lib/locale';
	import { ReportForm } from '$lib/forms';
	import { NarrowPage } from '$lib/ui';

	const { data } = $props();
</script>

<NarrowPage title={locale.reportTitle()}>
	<ReportForm address={data.address} />
</NarrowPage>
