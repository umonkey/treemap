<script lang="ts">
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import CheckInput from '$lib/ui/check-input/CheckInput.svelte';
	import SelectButton from '$lib/ui/SelectButton.svelte';
	import { selectorState } from './LayerSelector.svelte.ts';
</script>

<Dialog title="Select map base layer" variant="bottom">
	<div class="base-layers">
		<SelectButton
			value="basic"
			label="Details"
			active={selectorState.base === 'basic'}
			onClick={selectorState.setBase}
		/>
		<SelectButton
			value="light"
			label="Light"
			active={selectorState.base === 'light'}
			onClick={selectorState.setBase}
		/>
		<SelectButton
			value="google"
			label="Satellite"
			active={selectorState.base === 'google'}
			onClick={selectorState.setBase}
		/>
	</div>

	<h3>Select additional layers</h3>

	<div class="additional-layers">
		<CheckInput value={selectorState.drone} label="Drone" onChange={selectorState.toggleDrone} />
	</div>
</Dialog>

<style>
	.base-layers {
		display: flex;
		gap: 0.5rem;
		width: 100%;

		:global(button) {
			flex: 1;
		}
	}

	.additional-layers {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	h3 {
		font-size: 1.25rem;
		font-weight: 400;
		margin: 2rem 0 0.5rem;
	}
</style>
