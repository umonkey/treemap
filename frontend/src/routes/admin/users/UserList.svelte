<script lang="ts">
	import type { IUser } from '$lib/types';
	let { users } = $props<{ users: IUser[] }>();
</script>

<div class="user-list">
	<table>
		<thead>
			<tr>
				<th></th>
				<th>Name</th>
			</tr>
		</thead>
		<tbody>
			{#each users as user (user.id)}
				<tr>
					<td>
						{#if user.picture}
							<img src={user.picture} alt={user.name} class="user-pic" />
						{/if}
					</td>
					<td>
						<a href="/admin/users/{user.id}">{user.name}</a>
					</td>
				</tr>
			{/each}
		</tbody>
	</table>
</div>

<style>
	.user-list table {
		width: 100%;
		border-collapse: collapse;
	}

	.user-list th,
	.user-list td {
		padding: 8px;
		border-bottom: 1px solid #ddd;
		text-align: left;
	}

	.user-pic {
		width: 32px;
		height: 32px;
		border-radius: 50%;
		object-fit: cover;
	}
</style>
