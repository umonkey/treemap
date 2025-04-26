<script lang="ts">
	import { locale } from '$lib/locale';
	import { ReplaceForm } from '$lib/forms';
	import { Header } from '$lib/ui';

	const { data } = $props();
</script>

<svelte:head>
	<title>{locale.replaceTitle()}</title>
</svelte:head>

<Header title={locale.replaceTitle()} />

<div class="padded">
	<ReplaceForm id={data.id} />
</div>
