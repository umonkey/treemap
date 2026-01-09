<script lang="ts">
	import UserEditForm from '$lib/components/admin/UserEditForm/index.svelte';
	import { NarrowPage } from '$lib/ui';
	import { apiClient } from '$lib/api';
	import { goto } from '$app/navigation';
	import type { IUser } from '$lib/types';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();

	const onSave = async (updatedUser: Partial<IUser>) => {
		const res = await apiClient.updateUser(data.user.id, updatedUser);
		if (res.status === 200) {
			goto('/admin/users');
		} else {
			alert('Error updating user');
		}
	};
</script>

<NarrowPage>
	<h1>Edit User</h1>
	<UserEditForm user={data.user} {onSave} />
</NarrowPage>
