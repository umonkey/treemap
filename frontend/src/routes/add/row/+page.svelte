<script lang="ts">
	import { AddRowForm, Header, NarrowPage, AuthWrapper } from '$lib/ui';
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
	<title>{locale.addRowTitle()}</title>
</svelte:head>

<Header title={locale.addRowTitle()} />

<NarrowPage>
	<AuthWrapper>
		<AddRowForm start={data.start} end={data.end} />
	</AuthWrapper>
</NarrowPage>
