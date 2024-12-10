<script>
	export let value = '';
	export let placeholder = '';
	export let label = '';
	export let type = 'text';
	export let multiline = false;
	export let hint = '';
</script>

<div class="input">
	<label>
		<span>{label}</span>
		{#if multiline}
			<textarea bind:value {placeholder}>{value}</textarea>
		{:else}
			<input {type} bind:value {placeholder} />
		{/if}
	</label>

	{#if hint}
		<div class="hint">{hint}</div>
	{/if}
</div>

<style>
	label {
		display: block;
		margin-top: calc(2 * var(--gap));
	}

	span {
		display: block;
		margin-bottom: var(--gap);
	}

	input,
	textarea {
		width: 100%;
		padding: var(--gap);
		border: 1px solid var(--border);
		border-radius: var(--radius);
		box-sizing: border-box;
		background-color: transparent;
		border: 1px solid var(--sep-color);
		border-radius: 6px;
		color: var(--form-color);
		outline: none;
		line-height: 1.25em;
	}

	textarea {
		min-width: 100%;
		max-width: 100%;
		min-height: 100px;
	}

	.hint {
		color: var(--text-color-inactive);
		font-size: 0.85em;
		line-height: 125%;
		margin-top: var(--gap);
	}
</style>
