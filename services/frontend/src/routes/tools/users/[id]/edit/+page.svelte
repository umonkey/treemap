<script lang="ts">
	import { page } from '$app/state';
	import PageHeader from '$lib/ui/header/PageHeader.svelte';
	import Button from '$lib/ui/button/Button.svelte';
	import Buttons from '$lib/ui/buttons/Buttons.svelte';
	import { PageState } from './page.svelte';

	const pageState = new PageState();

	$effect(() => {
		if (page.params.id) {
			pageState.reload(page.params.id);
		}
	});

	const handleSubmit = (e: Event) => {
		e.preventDefault();
		pageState.handleSave();
	};
</script>

<svelte:head>
	{#if pageState.user}
		<title>User Editor: {pageState.user.name}</title>
	{/if}
</svelte:head>

<article>
	{#if pageState.loading}
		<p>Loading...</p>
	{:else if pageState.error}
		<p class="error">Error loading user: {pageState.error.description}</p>
	{:else if pageState.user}
		<PageHeader text="User Editor" />

		<form onsubmit={handleSubmit} class="user-edit-form">
			<div class="field">
				<label for="id">ID</label>
				<input type="text" id="id" value={pageState.user.id} disabled />
			</div>

			<div class="field">
				<label for="name">Name</label>
				<input type="text" id="name" bind:value={pageState.name} />
			</div>

			<div class="field">
				<label for="email">Email</label>
				<input type="text" id="email" value={pageState.user.email} disabled />
			</div>

			<Buttons>
				<Button type="submit" disabled={pageState.saving}>
					{pageState.saving ? 'Saving...' : 'Save'}
				</Button>
				<Button link="/tools/users/{pageState.user.id}" type="cancel">Cancel</Button>
			</Buttons>
		</form>
	{/if}
</article>

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
