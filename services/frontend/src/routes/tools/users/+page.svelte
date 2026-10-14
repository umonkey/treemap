<script lang="ts">
	import PageHeader from '$lib/ui/header/PageHeader.svelte';
	import { PageState } from './page.svelte';

	const pageState = new PageState();

	$effect(() => {
		pageState.reload();
	});
</script>

<svelte:head>
	<title>User List</title>
</svelte:head>

<article>
	<PageHeader text="User List" />
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
								<a href="/tools/users/{user.user.id}">{user.user.name}</a>
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
