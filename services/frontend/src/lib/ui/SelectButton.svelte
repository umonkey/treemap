<script lang="ts">
	const {
		value,
		label,
		active = false,
		onClick
	} = $props<{
		value: string;
		label: string;
		active?: boolean;
		onClick: (value: string) => void;
	}>();

	const handleClick = (e: Event) => {
		e.preventDefault();
		onClick(value);
	};
</script>

<button class:active={!!active} onclick={handleClick} type="button">{label}</button>

<style>
	button {
		border: none;
		padding: 0.5rem 0.75rem;
		margin: 0;
		cursor: pointer;
		background-color: var(--sep-color);

		&.active,
		&:hover {
			background-color: var(--text-color-inactive);
			color: var(--sep-color);
		}
	}
</style>
