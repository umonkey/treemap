<script lang="ts">
	import { locale } from '$lib/locale';
	import AuthWrapper from '$lib/ui/auth-wrapper/AuthWrapper.svelte';
	import Button from '$lib/ui/button/Button.svelte';
	import Buttons from '$lib/ui/buttons/Buttons.svelte';
	import Form from '$lib/ui/form/Form.svelte';
	import type { Snippet } from 'svelte';

	type Props = {
		children?: Snippet;
		canSave?: boolean;
		saving?: boolean;
		onSubmit: () => void;
		onCancel: () => void;
	};

	const { children, canSave = true, saving = false, onSubmit, onCancel }: Props = $props();

	const handleSubmit = () => {
		if (!canSave || saving) {
			return;
		}

		onSubmit();
	};

	const handleCancel = () => {
		if (saving) {
			return;
		}

		onCancel();
	};
</script>

<Form onSubmit={handleSubmit}>
	{#if children}
		<AuthWrapper>
			{@render children()}
		</AuthWrapper>
	{/if}

	<Buttons>
		<Button disabled={!canSave || saving}>{locale.editSave()}</Button>
		<Button type="cancel" onClick={handleCancel} disabled={saving}>{locale.editCancel()}</Button>
	</Buttons>
</Form>
