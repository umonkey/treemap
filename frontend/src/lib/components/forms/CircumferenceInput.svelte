<script>
	import TextInput from '$lib/components/forms/TextInput.svelte';
	export let value = '';
</script>

<TextInput
	label="Trunk circumference, cm"
	bind:value
	type="number"
	hint="Measure at chest level, aka 120 cm from the ground."
	help="https://github.com/KanachYerevan/kb/wiki/Measuring-trunk-circumference"
/>
