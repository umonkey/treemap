<script lang="ts">
	import { locale } from '$lib/locale';
	import { MoveForm } from '$lib/forms';
	import { NarrowPage } from '$lib/ui';
	import AuthWrapper from '$lib/components/auth/AuthWrapper.svelte';

	const { data } = $props();
</script>

<NarrowPage title={locale.measureTitle()}>
	<AuthWrapper>
		<MoveForm id={data.id} />
	</AuthWrapper>
</NarrowPage>
