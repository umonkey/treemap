<script lang="ts">
	import { locale } from '$lib/locale';
	import ReportForm from './ReportForm.svelte';
	import Dialog from '$lib/components/layout/Dialog.svelte';

	const { data } = $props();
</script>

<Dialog title={locale.reportTitle()}>
	<ReportForm address={data.address} />
</Dialog>
