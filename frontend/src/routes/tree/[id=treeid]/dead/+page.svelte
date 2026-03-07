<script lang="ts">
	import { locale } from '$lib/locale';
	import { NarrowPage } from '$lib/ui';
	import { TreeDeadForm } from '$lib/forms';
	import { routes } from '$lib/routes';

	const { data } = $props<{
		data: {
			id: string;
		};
	}>();
</script>

<NarrowPage title={locale.deadTitle()} back={routes.mapPreview(data.id)}>
	<TreeDeadForm id={data.id} />
</NarrowPage>
