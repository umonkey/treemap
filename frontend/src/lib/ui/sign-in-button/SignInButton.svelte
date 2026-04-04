<script lang="ts">
	import { config } from '$lib/env';
	import { locale } from '$lib/locale';
	import Button from '$lib/ui/button/Button.svelte';

	const onSignIn = () => {
		const qs = new URLSearchParams({
			client_id: config.authClientId,
			scope: 'openid email profile',
			response_type: 'token',
			redirect_uri: config.authCallback,
			response_mode: 'form_post',
			state: window.location.href

			/*
			gsiwebsdk: 'gis_attributes',
			prompt: 'select_account',
			service: 'lso',
			flowName: 'GeneralOAuthFlow',
			*/
		});

		const url = `https://accounts.google.com/o/oauth2/auth?${qs.toString()}`;
		window.location.assign(url);
	};
</script>

<Button onClick={onSignIn}>{locale.signInWithGoogle()}</Button>
