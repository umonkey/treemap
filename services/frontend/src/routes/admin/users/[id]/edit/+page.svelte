<script lang="ts">
	import { goto } from '$app/navigation';
	import { updateUser } from '$lib/api/users';
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import { showError } from '$lib/errors';
	import type { IUser } from '$lib/types';
	import type { PageData } from './$types';
	import UserEditForm from './UserEditForm.svelte';

	let { data }: { data: PageData } = $props();

	const onSave = async (updatedUser: Partial<IUser>) => {
		const res = await updateUser(data.user.id, updatedUser);

		if (res.status >= 200 && res.status < 300) {
			goto('/admin/users');
		} else {
			showError(res.error?.description || `Error ${res.status} updating user.`);
		}
	};
</script>

<Dialog title="User Editor">
	<UserEditForm user={data.user} {onSave} />
</Dialog>
