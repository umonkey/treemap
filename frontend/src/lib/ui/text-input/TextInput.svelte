<script lang="ts">
	import { HelpButton } from '$lib/ui';

	const {
		value = '',
		placeholder = '',
		label = '',
		type = 'text',
		multiline = false,
		hint = '',
		help,
		onChange
	} = $props<{
		value: string | number;
		placeholder: string;
		label: string;
		type: string;
		multiline: boolean;
		hint: string;
		help?: string;
		onChange: (value: string) => void;
	}>();

	const handleChange = (e: Event) => {
		if (e.target) {
			const em = e.target as HTMLInputElement;
			onChange(em.value ?? '');
		}
	};
</script>

<div class="input">
	<label>
		<span>{label}</span>
		{#if multiline}
			<textarea {placeholder} onchange={handleChange}>{value}</textarea>
		{:else}
			<div class="group">
				<input class="form" {type} {value} {placeholder} onchange={handleChange} />
				<HelpButton {help} />
			</div>
		{/if}
	</label>

	{#if hint}
		<div class="hint">{hint}</div>
	{/if}
</div>

<style>
	.group {
		display: flex;
		flex-direction: row;
		gap: var(--gap);
	}
</style>
