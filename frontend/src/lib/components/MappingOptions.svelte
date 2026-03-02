<script lang="ts">
	import AutoUploadCheckbox from '$lib/components/AutoUploadCheckbox.svelte';
</script>

<fieldset>
	<legend>Your mapping experience:</legend>
	<AutoUploadCheckbox />
</fieldset>

<style>
	fieldset {
		margin: 1rem 0;
		padding: 0.5rem 1rem;
		border-radius: 5px;
	}
</style>
