<script lang="ts">
	import Breadcrumbs from '$lib/components/admin/Breadcrumbs.svelte';
	import PageHeader from '$lib/ui/header/PageHeader.svelte';
</script>

<svelte:head>
	<title>Admin Dashboard</title>
</svelte:head>

<article>
	<PageHeader text="Admin Dashboard" />
	<Breadcrumbs items={[{ label: 'Home', href: '/' }, { label: 'Admin' }]} />
</article>
