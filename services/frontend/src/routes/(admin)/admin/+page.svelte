<script lang="ts">
	import Breadcrumbs from '$lib/components/admin/Breadcrumbs.svelte';
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
		<li><a href="/admin/users">Manage Users</a></li>
		<li><a href="/admin/sequences">StreetView Sequences</a></li>
	</ul>
</article>
