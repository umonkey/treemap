<script lang="ts">
	import { locale } from '$lib/locale';
	import { CircumferenceForm } from '$lib/forms';
	import { Header } from '$lib/ui';

	const { data } = $props();
</script>

<svelte:head>
	<title>{locale.measureTitle()}</title>
</svelte:head>

<Header title={locale.measureTitle()} />

<div class="padded measure">
	<CircumferenceForm id={data.id} />
</div>
