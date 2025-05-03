<script lang="ts">
	import { locale } from '$lib/locale';
	import { NarrowPage } from '$lib/ui';
	import { TreeDeadForm } from '$lib/forms';

	const { data } = $props<{
		data: {
			id: string;
		};
	}>();
</script>

<NarrowPage title={locale.deadTitle()}>
	<TreeDeadForm id={data.id} />
</NarrowPage>
