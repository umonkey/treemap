<script lang="ts">
	import Breadcrumbs from '$lib/components/admin/Breadcrumbs.svelte';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();
</script>

<svelte:head>
	<title>User List</title>
</svelte:head>

<article>
	<header>
		<h1>User List</h1>

		<Breadcrumbs items={[{ label: 'Admin', href: '/admin' }, { label: 'Users' }]} />
	</header>
	{#if data.error}
		<p class="error">Error loading users: {data.error.description}</p>
	{:else}
		<div class="user-list">
			<table>
				<thead>
					<tr>
						<th></th>
						<th>Name</th>
					</tr>
				</thead>
				<tbody>
					{#each data.users as user (user.id)}
						<tr>
							<td>
								{#if user.picture}
									<img src={user.picture} alt="" referrerpolicy="no-referrer" class="user-pic" />
								{/if}
							</td>
							<td width="100%">
								<a href="/admin/users/{user.id}">{user.name}</a>
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	{/if}
</article>

<style>
	.error {
		color: red;
	}

	.user-list table {
		width: 100%;
		border-collapse: collapse;
	}

	.user-list th,
	.user-list td {
		padding: 8px;
		border-bottom: 1px solid light-dark(#ddd, #444);
		text-align: left;
	}

	.user-pic {
		width: 32px;
		height: 32px;
		border-radius: 50%;
		object-position: center;
		object-fit: cover;
		overflow: hidden;
	}
</style>
