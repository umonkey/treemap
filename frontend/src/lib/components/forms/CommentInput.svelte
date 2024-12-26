<script>
	import { locale } from '$lib/locale';
	import TextInput from '$lib/components/forms/TextInput.svelte';
	export let value = '';
</script>

<TextInput label={locale.commentLabel()} bind:value multiline={true} hint={locale.commentHint()} />
