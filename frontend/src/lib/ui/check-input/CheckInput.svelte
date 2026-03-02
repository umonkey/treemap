<script lang="ts">
	let {
		value = $bindable(false),
		label = '',
		hint = '',
		disabled = false,
		id,
		onChange
	} = $props<{
		value: boolean;
		label?: string;
		hint?: string;
		disabled?: boolean;
		id?: string;
		onChange?: (value: boolean) => void;
	}>();

	const handleChange = (e: Event) => {
		const target = e.target as HTMLInputElement;
		if (onChange) {
			onChange(target.checked);
		}
	};
</script>

{#if label || hint}
	<label class:disabled>
		<div class="switch">
			<input type="checkbox" {id} bind:checked={value} {disabled} onchange={handleChange} />
			<span class="slider"></span>
		</div>
		<span>{label}</span>
	</label>
{:else}
	<label class="standalone" class:disabled>
		<div class="switch">
			<input type="checkbox" {id} bind:checked={value} {disabled} onchange={handleChange} />
			<span class="slider"></span>
		</div>
	</label>
{/if}

<style>
	label {
		display: flex;
		flex-direction: row;
		gap: 1rem;
		align-items: center;
		cursor: pointer;
	}

	.switch {
		position: relative;
		display: inline-block;
		width: 44px;
		height: 24px;
		vertical-align: middle;
	}

	.switch input {
		opacity: 0;
		width: 0;
		height: 0;
	}

	.slider {
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background-color: light-dark(#bfc7d9, #333c4e);
		transition: 0.3s;
		border-radius: 24px;
	}

	.slider:before {
		position: absolute;
		content: '';
		height: 20px;
		width: 20px;
		left: 2px;
		bottom: 2px;
		background-color: white;
		transition: 0.3s;
		border-radius: 50%;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);
	}

	input:checked + .slider {
		background-color: #4cd964; /* iOS Green */
	}

	input:focus + .slider {
		box-shadow: 0 0 1px #4cd964;
	}

	input:checked + .slider:before {
		transform: translateX(20px);
	}

	.disabled {
		opacity: 0.6;
		pointer-events: none;
	}

	.standalone {
		display: block;
	}
</style>
