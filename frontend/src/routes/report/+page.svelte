<script lang="ts">
	import { locale } from '$lib/locale';
	import { ReportForm } from '$lib/forms';
	import { NarrowPage } from '$lib/ui';
</script>

<NarrowPage title={locale.reportTitle()}>
	<ReportForm />
</NarrowPage>
