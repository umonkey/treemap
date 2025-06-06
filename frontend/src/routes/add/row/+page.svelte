<script lang="ts">
	import { AddForm } from '$lib/forms';
	import { Header, NarrowPage, AuthWrapper } from '$lib/ui';
	import { locale } from '$lib/locale';

	import type { ILatLng } from '$lib/types';

	const { data } = $props<{
		data: {
			start: ILatLng;
			end: ILatLng;
		};
	}>();
</script>

<svelte:head>
	<title>{locale.addTitle()}</title>
</svelte:head>

<Header title={locale.addTitle()} />

<NarrowPage>
	<AuthWrapper>
		<AddForm lat={data.lat} lng={data.lng} />
	</AuthWrapper>
</NarrowPage>
