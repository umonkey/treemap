<script lang="ts">
	import Breadcrumbs from '$lib/components/admin/Breadcrumbs.svelte';
	import { hasPermission } from '$lib/stores/authStore';
</script>

<svelte:head>
	<title>Admin Dashboard</title>
</svelte:head>

<article>
	<header>
		<h1>Admin Dashboard</h1>
		<Breadcrumbs items={[{ label: 'Home', href: '/' }, { label: 'Admin' }]} />
	</header>
	<ul>
		{#if $hasPermission('user:manage')}
			<li><a href="/admin/users">Manage Users</a></li>
		{/if}
		{#if $hasPermission('pano:edit')}
			<li><a href="/admin/sequences">StreetView Sequences</a></li>
		{/if}
	</ul>
</article>
