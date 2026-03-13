<script lang="ts">
	import AddForm from './AddForm.svelte';
	import { Header, NarrowPage } from '$lib/ui';
	import { locale } from '$lib/locale';

	const { data } = $props<{
		data: {
			lat: number;
			lng: number;
		};
	}>();
</script>

<svelte:head>
	<title>{locale.addTitle()}</title>
</svelte:head>

<Header title={locale.addTitle()} />

<NarrowPage>
	<AddForm lat={data.lat} lng={data.lng} />
</NarrowPage>
