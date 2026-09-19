<script lang="ts">
	import Breadcrumbs from '$lib/components/admin/Breadcrumbs.svelte';
	import ToolList from '$lib/components/tools/ToolList.svelte';
	import PageHeader from '$lib/ui/header/PageHeader.svelte';
</script>

<svelte:head>
	<title>Admin Dashboard</title>
</svelte:head>

<article>
	<PageHeader text="Admin Dashboard" />
	<Breadcrumbs items={[{ label: 'Home', href: '/' }, { label: 'Admin' }]} />
	<ToolList />
</article>
