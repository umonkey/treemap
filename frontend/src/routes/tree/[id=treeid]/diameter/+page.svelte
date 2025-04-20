<script lang="ts">
	import { locale } from '$lib/locale';
	import { CrownDiameterForm } from '$lib/forms';
	import { Header } from '$lib/ui';

	const { data } = $props();
</script>

<svelte:head>
	<title>{locale.measureTitle()}</title>
</svelte:head>

<Header title={locale.measureTitle()} />

<div class="padded measure">
	<CrownDiameterForm id={data.id} />
</div>
