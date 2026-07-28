<script lang="ts">
	import Breadcrumbs from '$lib/components/admin/Breadcrumbs.svelte';
	import PageHeader from '$lib/ui/header/PageHeader.svelte';
	import { hasPermission } from '$lib/stores/authStore';
</script>

<svelte:head>
	<title>Admin Dashboard</title>
</svelte:head>

<article>
	<PageHeader text="Admin Dashboard" />
	<Breadcrumbs items={[{ label: 'Home', href: '/' }, { label: 'Admin' }]} />
	<ul>
		{#if $hasPermission('user:manage')}
			<li><a href="/admin/users">Manage Users</a></li>
		{/if}
		{#if $hasPermission('pano:edit')}
			<li><a href="/admin/panoramas">Manage Panoramas</a></li>
		{/if}
	</ul>
</article>
