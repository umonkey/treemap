<script lang="ts">
	import type { IUser } from '$lib/types';
	import Buttons from '$lib/ui/buttons/Buttons.svelte';
	import Button from '$lib/ui/button/Button.svelte';
	import './styles.css';

	let { user }: { user: IUser } = $props();
</script>

<div class="user-details">
	<h1>{user.name}</h1>
	{#if user.picture}
		<img src={user.picture} alt={user.name} class="user-pic-large" />
	{/if}

	<dl>
		<dt>ID</dt>
		<dd>{user.id}</dd>

		<dt>Email</dt>
		<dd>{user.email}</dd>

		<dt>Trees Count</dt>
		<dd>{user.trees_count}</dd>

		<dt>Comments Count</dt>
		<dd>{user.comments_count}</dd>

		<dt>Updates Count</dt>
		<dd>{user.updates_count}</dd>

		<dt>Files Count</dt>
		<dd>{user.files_count}</dd>
	</dl>

	<Buttons>
		<Button link="/admin/users/{user.id}/edit">Edit</Button>
		<Button link="/admin/users" type="cancel">Back to List</Button>
	</Buttons>
</div>
