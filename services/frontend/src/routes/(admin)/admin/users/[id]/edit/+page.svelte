<script lang="ts">
	import { goto } from '$app/navigation';
	import { updateUser } from '$lib/api/users';
	import { showError } from '$lib/errors';
	import type { IUser } from '$lib/types';
	import type { PageData } from './$types';
	import UserEditForm from './UserEditForm.svelte';
	import Breadcrumbs from '$lib/components/admin/Breadcrumbs.svelte';

	let { data }: { data: PageData } = $props();

	const onSave = async (updatedUser: Partial<IUser>) => {
		const res = await updateUser(data.user.user.id, updatedUser);

		if (res.status >= 200 && res.status < 300) {
			goto('/admin/users');
		} else {
			showError(res.error?.description || `Error ${res.status} updating user.`);
		}
	};
</script>

<svelte:head>
	<title>User Editor: {data.user.user.name}</title>
</svelte:head>

<article>
	<header>
		<h1>User Editor</h1>

		<Breadcrumbs
			items={[
				{ label: 'Admin', href: '/admin' },
				{ label: 'Users', href: '/admin/users' },
				{ label: data.user.user.name, href: `/admin/users/${data.user.user.id}` },
				{ label: 'Edit' }
			]}
		/>
	</header>
	<UserEditForm user={data.user.user} {onSave} />
</article>
