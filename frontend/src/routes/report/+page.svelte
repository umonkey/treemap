<script lang="ts">
	import { page } from '$app/stores';
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import { locale } from '$lib/locale';
	import Form from '$lib/ui/form/Form.svelte';
	import StreetInput from '$lib/ui/street-input/StreetInput.svelte';
	import StreetReport from '$lib/ui/street-report/StreetReport.svelte';
	import { hooks } from './hooks';

	const address = $derived($page.url.searchParams.get('address'));
	const { report, handleStreetChange, reload } = hooks();

	$effect(() => {
		reload(address ?? null);
	});
</script>

<Dialog title={locale.reportTitle()}>
	<div class="report">
		<Form>
			<StreetInput value={address} onChange={handleStreetChange} />
		</Form>
	</div>

	{#if $report}
		<StreetReport data={$report} />
	{/if}
</Dialog>
