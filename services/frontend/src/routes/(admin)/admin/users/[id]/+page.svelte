<script lang="ts">
	import Breadcrumbs from '$lib/components/admin/Breadcrumbs.svelte';
	import Button from '$lib/ui/button/Button.svelte';
	import Buttons from '$lib/ui/buttons/Buttons.svelte';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();
	const user = $derived(data.user);
</script>

<svelte:head>
	<title>User Properties: {user.user.name}</title>
</svelte:head>

<article>
	<header>
		<h1>User Properties</h1>

		<Breadcrumbs
			items={[
				{ label: 'Admin', href: '/admin' },
				{ label: 'Users', href: '/admin/users' },
				{ label: user.user.name }
			]}
		/>
	</header>
	<div class="user-details">
		<h2>{user.user.name}</h2>
		{#if user.user.picture}
			<img src={user.user.picture} alt={user.user.name} class="user-pic-large" />
		{/if}

		<dl>
			<dt>ID</dt>
			<dd>{user.user.id}</dd>

			<dt>Email</dt>
			<dd>{user.user.email}</dd>

			<dt>Trees Count</dt>
			<dd>{user.user.trees_count}</dd>

			<dt>Comments Count</dt>
			<dd>{user.user.comments_count}</dd>

			<dt>Updates Count</dt>
			<dd>{user.user.updates_count}</dd>

			<dt>Files Count</dt>
			<dd>{user.user.files_count}</dd>

			<dt>Roles</dt>
			<dd>{(user.roles || []).join(', ')}</dd>

			<dt>Permissions</dt>
			<dd>
				<ul>
					{#each user.permissions || [] as perm}
						<li><code>{perm}</code></li>
					{/each}
				</ul>
			</dd>
		</dl>

		<Buttons>
			<Button link="/admin/users/{user.user.id}/edit">Edit</Button>
			<Button link="/admin/users" type="cancel">Back to List</Button>
		</Buttons>
	</div>
</article>

<style>
	.user-details dl {
		display: grid;
		grid-template-columns: max-content auto;
		gap: 10px 20px;
	}

	.user-pic-large {
		max-width: 200px;
		border-radius: 8px;
		margin-bottom: 20px;
	}
</style>
