<script lang="ts">
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
</script>

<div>
	<button {disabled} type="button" class="button" onclick={onClick}>Add photos</button>
	<input type="file" accept="image/jpeg" bind:this={input} onchange={onChange} multiple />
</div>

<style>
	input {
		display: none;
	}
</style>
