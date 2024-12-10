<script>
	export let label = 'Submit';
	export let type = 'button';
	export let onClick = () => {};
</script>

<button type="button" class={type} onclick={onClick}>{label}</button>

<style>
	button {
		background-color: var(--form-border);
		border: 1px solid var(--form-border);
		color: var(--form-color);
		outline: none;
		padding: 0.5rem 1rem;
		cursor: pointer;
		border-radius: 6px;
	}

	button.cancel {
		background-color: transparent;
		color: var(--form-border);
	}
</style>
