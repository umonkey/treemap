<script lang="ts">
	import { locale } from '$lib/locale';

	let input;

	export let disabled = false;
	export let onFileSelected: (files: File[]) => void;

	const onClick = () => {
		input.click();
	};

	const onChange = (event) => {
		if (event.target.files.length > 0) {
			onFileSelected(event.target.files);
		}
	};

	const isPWA = window.matchMedia('(display-mode: standalone)').matches;
</script>

<div>
	{#if isPWA}
		<button {disabled} type="button" class="button" onclick={onClick}
			>{locale.photoTake()}</button
		>
		<input
			type="file"
			accept="image/jpeg"
			bind:this={input}
			onchange={onChange}
			capture="environment"
		/>
	{:else}
		<button {disabled} type="button" class="button" onclick={onClick}>{locale.photoSelect()}</button
		>
		<input type="file" accept="image/jpeg" bind:this={input} onchange={onChange} multiple />
	{/if}
</div>

<style>
	input {
		display: none;
	}
</style>
