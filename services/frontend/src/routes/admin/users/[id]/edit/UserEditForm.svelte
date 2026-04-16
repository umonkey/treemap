<script lang="ts">
	import type { IUser } from '$lib/types';
	import Button from '$lib/ui/button/Button.svelte';
	import Buttons from '$lib/ui/buttons/Buttons.svelte';

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

<style>
	.user-edit-form .field {
		margin-bottom: 15px;
	}

	.user-edit-form label {
		display: block;
		margin-bottom: 5px;
		font-weight: bold;
	}

	.user-edit-form input {
		width: 100%;
		padding: 8px;
		border: 1px solid var(--input-border);
		border-radius: 4px;
	}

	.user-edit-form input:disabled {
		background-color: var(--disabled-input-bg);
		color: var(--disabled-input-fg);
	}
</style>
