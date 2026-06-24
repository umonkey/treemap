<script lang="ts">
	import Breadcrumbs from '$lib/components/admin/Breadcrumbs.svelte';
	import Button from '$lib/ui/button/Button.svelte';
	import Buttons from '$lib/ui/buttons/Buttons.svelte';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();
	const user = $derived(data.user);
</script>

<svelte:head>
	<title>User Properties: {user.name}</title>
</svelte:head>

<article>
	<header>
		<h1>User Properties</h1>

		<Breadcrumbs
			items={[
				{ label: 'Admin', href: '/admin' },
				{ label: 'Users', href: '/admin/users' },
				{ label: user.name }
			]}
		/>
	</header>
	<div class="user-details">
		<h2>{user.name}</h2>
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
