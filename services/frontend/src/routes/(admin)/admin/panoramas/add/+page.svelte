<script lang="ts">
	import { createPanorama } from '$lib/api/panoramas';
	import { goto } from '$app/navigation';
	import Breadcrumbs from '$lib/components/admin/Breadcrumbs.svelte';
	import AuthWrapper from '$lib/ui/auth-wrapper/AuthWrapper.svelte';
	import Button from '$lib/ui/button/Button.svelte';
	import TextInput from '$lib/ui/text-input/TextInput.svelte';
	import Form from '$lib/ui/form/Form.svelte';
	import Buttons from '$lib/ui/buttons/Buttons.svelte';

	let title = $state('');
	let isBusy = $state(false);
	let error = $state('');

	async function handleSubmit() {
		if (!title || isBusy) return;
		isBusy = true;
		error = '';

		const res = await createPanorama({ title });
		if (res.status === 201 && res.data) {
			goto(`/admin/panoramas/${res.data.id}`);
		} else {
			error = res.error?.description || 'Failed to create panorama';
			isBusy = false;
		}
	}
</script>

<svelte:head>
	<title>Add Panorama</title>
</svelte:head>

<AuthWrapper permission="pano:edit">
	<article>
		<header>
			<h1>Add Panorama</h1>
			<Breadcrumbs
				items={[
					{ label: 'Admin', href: '/admin' },
					{ label: 'Panoramas', href: '/admin/panoramas' },
					{ label: 'Add' }
				]}
			/>
		</header>

		<Form onSubmit={handleSubmit}>
			<TextInput
				label="Title"
				value={title}
				onChange={(v) => (title = v)}
				placeholder="Enter panorama title"
			/>

			{#if error}
				<p class="error">{error}</p>
			{/if}

			<Buttons>
				<Button type="submit" disabled={isBusy || !title}>Create Panorama</Button>
				<Button link="/admin/panoramas" type="cancel">Cancel</Button>
			</Buttons>
		</Form>
	</article>
</AuthWrapper>

<style>
	.error {
		color: red;
	}
</style>
