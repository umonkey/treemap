<script lang="ts">
	import ToolGrid from '$lib/components/tools/ToolGrid.svelte';
	import PageHeader from '$lib/ui/header/PageHeader.svelte';
	import { locale } from '$lib/locale';
</script>

<svelte:head>
	<title>Tools — {locale.appTitle()}</title>
</svelte:head>

<article>
	<PageHeader text="Additional tools" />

	<p class="description">
		Here you can find various additional tools that this application provides for data management
		and access.
	</p>

	<ToolGrid onSelect={() => {}} />
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
