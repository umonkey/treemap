<script lang="ts">
	import { Dialog } from '$lib/ui';
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import AddRowForm from './AddRowForm.svelte';
	import { locale } from '$lib/locale';

	import type { ILatLng } from '$lib/types';

	const { data } = $props<{
		data: {
			start: ILatLng;
			end: ILatLng;
		};
	}>();
</script>

<Dialog title={locale.addRowTitle()}>
	<AddRowForm start={data.start} end={data.end} />
</Dialog>
