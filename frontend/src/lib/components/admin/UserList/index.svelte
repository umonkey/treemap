<script lang="ts">
	import type { IUser } from '$lib/types';
	import './styles.css';

	let { users }: { users: IUser[] } = $props();
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
