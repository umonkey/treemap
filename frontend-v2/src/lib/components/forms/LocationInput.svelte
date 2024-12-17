<script lang="ts">
	const { value, hint = undefined } = $props();

	const formatLocation = (value: number[]): string => {
		return value.join(', ');
	};
</script>

<div class="input">
	<label>
		<span>Location</span>
		<input type="text" value={formatLocation(value)} />
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

	input {
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

	.hint {
		color: var(--text-color-inactive);
		font-size: 0.85em;
		line-height: 125%;
		margin-top: var(--gap);
	}
</style>
