<script lang="ts">
	import { locale } from '$lib/locale';
	import { MoveForm } from '$lib/forms';
	import { NarrowPage, AuthWrapper } from '$lib/ui';

	const { data } = $props();
</script>

<NarrowPage title={locale.measureTitle()}>
	<AuthWrapper>
		<MoveForm id={data.id} />
	</AuthWrapper>
</NarrowPage>
