<script lang="ts">
	import { signInWithGoogle } from '$lib/auth';
	import { locale } from '$lib/locale';
	import Button from '$lib/ui/button/Button.svelte';
</script>

<Button onClick={signInWithGoogle}>{locale.signInWithGoogle()}</Button>
