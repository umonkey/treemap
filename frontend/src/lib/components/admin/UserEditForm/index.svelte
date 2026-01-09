<script lang="ts">
	import type { IUser } from '$lib/types';
	import { Button, Buttons } from '$lib/ui';
	import './styles.css';

	let { user, onSave }: { user: IUser; onSave: (data: Partial<IUser>) => void } = $props();

	let name = $state(user.name);

	const handleSubmit = (e: Event) => {
		e.preventDefault();
		onSave({ ...user, name });
	};
</script>

<form onsubmit={handleSubmit} class="user-edit-form">
	<div class="field">
		<label for="id">ID</label>
		<input type="text" id="id" value={user.id} disabled />
	</div>

	<div class="field">
		<label for="name">Name</label>
		<input type="text" id="name" bind:value={name} />
	</div>

	<div class="field">
		<label for="email">Email</label>
		<input type="text" id="email" value={user.email} disabled />
	</div>

	<Buttons>
		<Button type="submit">Save</Button>
		<Button link="/admin/users/{user.id}" type="cancel">Cancel</Button>
	</Buttons>
</form>
