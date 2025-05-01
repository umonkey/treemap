<script lang="ts">
	import { EditForm } from '$lib/forms';
	import { Header } from '$lib/ui';
	import { locale } from '$lib/locale';
	import AuthWrapper from '$lib/components/auth/AuthWrapper.svelte';

	const { data } = $props();
</script>

<svelte:head>
	<title>{locale.editTitle()}</title>
</svelte:head>

<Header title={locale.editTitle()} />

<AuthWrapper>
	<EditForm id={data.id} />
</AuthWrapper>
