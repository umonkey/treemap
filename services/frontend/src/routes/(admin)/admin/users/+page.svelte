<script lang="ts">
	import Breadcrumbs from '$lib/components/admin/Breadcrumbs.svelte';
	import AuthWrapper from '$lib/ui/auth-wrapper/AuthWrapper.svelte';
	import { pageState } from './page.svelte';

	$effect(() => {
		pageState.reload();
	});
</script>

<svelte:head>
	<title>User List</title>
</svelte:head>

<AuthWrapper permission="user:manage">
	<article>
		<header>
			<h1>User List</h1>

			<Breadcrumbs items={[{ label: 'Admin', href: '/admin' }, { label: 'Users' }]} />
		</header>
		{#if pageState.loading}
			<p>Loading...</p>
		{:else if pageState.error}
			<p class="error">Error loading users: {pageState.error.description}</p>
		{:else}
			<div class="user-list">
				<table>
					<thead>
						<tr>
							<th></th>
							<th>Name</th>
							<th>Roles</th>
						</tr>
					</thead>
					<tbody>
						{#each pageState.users as user (user.user.id)}
							<tr>
								<td>
									{#if user.user.picture}
										<img
											src={user.user.picture}
											alt=""
											referrerpolicy="no-referrer"
											class="user-pic"
										/>
									{/if}
								</td>
								<td width="100%">
									<a href="/admin/users/{user.user.id}">{user.user.name}</a>
								</td>
								<td>
									<div class="roles">
										{#each user.roles || [] as role}
											<span class="role">{role}</span>
										{/each}
									</div>
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		{/if}
	</article>
</AuthWrapper>

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

	.roles {
		display: flex;
		gap: 4px;
	}

	.role {
		background-color: #eee;
		padding: 2px 6px;
		border-radius: 4px;
		font-size: 12px;
		white-space: nowrap;
		color: #333;
	}
</style>
