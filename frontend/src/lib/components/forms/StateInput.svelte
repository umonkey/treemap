<script lang="ts">
	import HelpButton from '$lib/components/forms/HelpButton.svelte';
	import { locale } from '$lib/locale';

	let { value = $bindable(''), label = true }: {
		value: string|null;
		label: boolean;
	} = $props();
</script>

<div class="input">
	<label>
		{#if label}<span>{locale.stateLabel()}</span>{/if}
		<div class="group">
			<select bind:value>
				<option value="">{locale.stateUnknown()}</option>
				<option value="healthy">{locale.stateHealthy()}</option>
				<option value="sick">{locale.stateSick()}</option>
				<option value="deformed">{locale.stateDeformed()}</option>
				<option value="dead">{locale.stateDead()}</option>
				<option value="gone">{locale.stateGone()}</option>
				<option value="stomp">{locale.stateStomp()}</option>
			</select>
			<HelpButton help="https://github.com/KanachYerevan/kb/wiki/Understanding-tree-state" />
		</div>
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

	.group {
		display: flex;
		flex-direction: row;
		gap: var(--gap);
	}
</style>
