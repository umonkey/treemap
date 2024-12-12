<script lang="ts">
	export let value = '';
</script>

<div class="input">
	<label>
		<span>State</span>
		<select bind:value>
			<option value="">(unknown)</option>
			<option value="healthy">healthy</option>
			<option value="sick">sick</option>
			<option value="deformed">deformed</option>
			<option value="gone">gone</option>
			<option value="stomp">stomp</option>
		</select>
	</label>
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

	select {
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
</style>
