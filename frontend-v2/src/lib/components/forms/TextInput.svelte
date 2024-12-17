<script lang="ts">
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
			<input class="form" {type} bind:value {placeholder} />
		{/if}
	</label>

	{#if hint}
		<div class="hint">{hint}</div>
	{/if}
</div>
