<script lang="ts">
	import { EditForm } from '$lib/forms';
	import { NarrowPage, AuthWrapper } from '$lib/ui';
	import { locale } from '$lib/locale';
	import { routes } from '$lib/routes';

	const { data } = $props();
</script>

<NarrowPage title={locale.editTitle()} back={routes.mapPreview(data.id)}>
	<AuthWrapper>
		<EditForm id={data.id} />
	</AuthWrapper>
</NarrowPage>
