<script lang="ts">
	import { page } from '$app/stores';
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import { locale } from '$lib/locale';
	import Form from '$lib/ui/form/Form.svelte';
	import StreetInput from '$lib/ui/street-input/StreetInput.svelte';
	import StreetReport from './StreetReport.svelte';
	import { pageState } from './page.svelte';

	const address = $derived($page.url.searchParams.get('address'));

	$effect(() => {
		pageState.reload(address ?? null);
	});
</script>

<Dialog title={locale.reportTitle()}>
	<div class="report">
		<Form>
			<StreetInput value={address} onChange={pageState.handleStreetChange} />
		</Form>
	</div>

	{#if pageState.report}
		<StreetReport data={pageState.report} />
	{/if}
</Dialog>
