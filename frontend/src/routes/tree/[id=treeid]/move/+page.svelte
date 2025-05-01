<script lang="ts">
	import { locale } from '$lib/locale';
	import { MoveForm } from '$lib/forms';
	import { Header } from '$lib/ui';
	import AuthWrapper from '$lib/components/auth/AuthWrapper.svelte';

	const { data } = $props();
</script>

<svelte:head>
	<title>{locale.measureTitle()}</title>
</svelte:head>

<Header title={locale.measureTitle()} />

<div class="padded measure">
	<AuthWrapper>
		<MoveForm id={data.id} />
	</AuthWrapper>
</div>
