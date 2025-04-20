<script lang="ts">
	import { locale } from '$lib/locale';
	import { Header } from '$lib/ui';
	import { TreeDeadForm } from '$lib/forms';

	const { data } = $props<{
		data: {
			id: string;
		};
	}>();
</script>

<svelte:head>
	<title>{locale.deadTitle()}</title>
</svelte:head>

<Header title={locale.deadTitle()} />

<div class="padded measure">
	<TreeDeadForm id={data.id} />
</div>
