<script lang="ts">
import { AUTH_CALLBACK, AUTH_CLIENT_ID } from "$lib/env";
import { locale } from "$lib/locale";

const onSignIn = () => {
	const qs = new URLSearchParams({
		client_id: AUTH_CLIENT_ID,
		scope: "openid email profile",
		response_type: "token",
		redirect_uri: AUTH_CALLBACK,
		response_mode: "form_post",
		state: window.location.href,

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

<button class="button" type="button" on:click={onSignIn}>{locale.signInWithGoogle()}</button>
