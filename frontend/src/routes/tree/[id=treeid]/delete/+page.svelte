<script lang="ts">
	import { locale } from '$lib/locale';
	import { NarrowPage } from '$lib/ui';
	import { DeleteTreeForm } from '$lib/forms';

	const { data } = $props<{
		data: {
			id: string;
		};
	}>();
</script>

<NarrowPage title={locale.deleteTreeTitle()}>
	<DeleteTreeForm id={data.id} />
</NarrowPage>
