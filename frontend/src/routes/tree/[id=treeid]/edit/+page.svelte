<script lang="ts">
	import { EditForm } from '$lib/forms';
	import { NarrowPage, AuthWrapper } from '$lib/ui';
	import { locale } from '$lib/locale';

	const { data } = $props();
</script>

<NarrowPage title={locale.editTitle()}>
	<AuthWrapper>
		<EditForm id={data.id} />
	</AuthWrapper>
</NarrowPage>
