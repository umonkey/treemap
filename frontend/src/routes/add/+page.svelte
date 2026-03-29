<script lang="ts">
	import AddForm from './AddForm.svelte';
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import { locale } from '$lib/locale';

	const { data } = $props<{
		data: {
			lat: number;
			lng: number;
		};
	}>();
</script>

<Dialog title={locale.addTitle()}>
	<AddForm lat={data.lat} lng={data.lng} />
</Dialog>
