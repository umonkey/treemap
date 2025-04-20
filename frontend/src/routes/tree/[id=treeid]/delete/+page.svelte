<script lang="ts">
	import { locale } from '$lib/locale';
	import { Header } from '$lib/ui';
	import { DeleteTreeForm } from '$lib/forms';

	const { data } = $props<{
		data: {
			id: string;
		};
	}>();
</script>

<svelte:head>
	<title>{locale.deleteTreeTitle()}</title>
</svelte:head>

<Header title={locale.deleteTreeTitle()} />

<div class="padded measure">
	<DeleteTreeForm id={data.id} />
</div>
