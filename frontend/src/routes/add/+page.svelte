<script lang="ts">
	import AuthWrapper from '$lib/components/auth/AuthWrapper.svelte';
	import { AddForm } from '$lib/forms';
	import { Header } from '$lib/ui';
	import { locale } from '$lib/locale';

	const { data } = $props<{
		data: {
			lat: number;
			lng: number;
		};
	}>();
</script>

<svelte:head>
	<title>{locale.addTitle()}</title>
</svelte:head>

<Header title={locale.addTitle()} />

<div class="form">
	<AuthWrapper>
		<AddForm lat={data.lat} lng={data.lng} />
	</AuthWrapper>
</div>

<style>
	.form {
		padding: 0 var(--gap) var(--gap);
	}
</style>
