<script lang="ts">
	import { locale } from '$lib/locale';
	import { NarrowPage } from '$lib/ui';
	import DeleteTreeForm from './DeleteTreeForm.svelte';
	import { routes } from '$lib/routes';

	const { data } = $props<{
		data: {
			id: string;
		};
	}>();
</script>

<NarrowPage title={locale.deleteTreeTitle()} back={routes.mapPreview(data.id)}>
	<DeleteTreeForm id={data.id} />
</NarrowPage>
