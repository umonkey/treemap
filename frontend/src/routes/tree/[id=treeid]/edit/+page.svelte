<script lang="ts">
	import { EditForm } from '$lib/forms';
	import { NarrowPage } from '$lib/ui';
	import { locale } from '$lib/locale';
	import AuthWrapper from '$lib/components/auth/AuthWrapper.svelte';

	const { data } = $props();
</script>

<NarrowPage title={locale.editTitle()}>
	<AuthWrapper>
		<EditForm id={data.id} />
	</AuthWrapper>
</NarrowPage>
