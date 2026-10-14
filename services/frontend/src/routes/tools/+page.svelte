<script lang="ts">
	import ToolList from '$lib/components/tools/ToolList.svelte';
	import PageHeader from '$lib/ui/header/PageHeader.svelte';
	import { locale as appLocale } from '$lib/locale';
	import { locale } from './lang';
</script>

<svelte:head>
	<title>{locale.title()} — {appLocale.appTitle()}</title>
</svelte:head>

<article>
	<PageHeader text={locale.heading()} />

	<p class="description">{locale.description()}</p>

	<ToolList />
</article>

<style>
	article {
		display: flex;
		flex-direction: column;
		gap: calc(var(--gap) * 2);
	}

	.description {
		margin: 0;
		line-height: 1.5;
		opacity: 0.75;
	}
</style>
