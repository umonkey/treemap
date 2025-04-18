<script lang="ts">
	const {
		label,
		type = 'submit',
		onClick,
		disabled = false
	} = $props<{
		label: string;
		type: 'submit' | 'button' | 'reset';
		onClick: () => void;
		disabled?: boolean;
	}>();

	const className = `button ${type}`;
</script>

<button type="button" disabled={!!disabled} class={className} onclick={onClick}>{label}</button>
