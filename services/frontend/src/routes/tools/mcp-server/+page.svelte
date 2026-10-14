<script lang="ts">
	import { locale as appLocale } from '$lib/locale';
	import PageHeader from '$lib/ui/header/PageHeader.svelte';
	import { locale } from './lang';
</script>

<svelte:head>
	<title>{locale.title()} — {appLocale.appTitle()}</title>
</svelte:head>

<article>
	<PageHeader text={locale.heading()} />

	<p>{locale.intro()}</p>
	<p>{locale.readOnly()}</p>
	<p>{locale.coverage()}</p>

	<h2>{locale.endpoint()}</h2>
	<pre><code>https://api.treemaps.app/mcp</code></pre>

	<p>
		<a
			href="https://github.com/umonkey/treemap/blob/master/docs/Remote-MCP.md"
			target="_blank"
			rel="noopener noreferrer">{locale.documentation()}</a
		>
	</p>
</article>

<style>
	p {
		line-height: 1.5;
	}

	pre {
		overflow-x: auto;
		background-color: rgba(128, 128, 128, 0.2);
		padding: 1rem;
	}
</style>
