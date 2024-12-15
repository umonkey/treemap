<script lang="ts">
	/**
	 * This uses Google's Sign-In HTML interface.
	 *
	 * We need to specify the call-back URI for this to work.
	 */
	import { googleCallbackHandler } from '$lib/utils/auth';
	import { AUTH_CALLBACK, AUTH_CLIENT_ID } from '$lib/env';

	window.onSignIn = googleCallbackHandler;

	const onSignIn = () => {
		const qs = new URLSearchParams({
			client_id: AUTH_CLIENT_ID,
			scope: 'openid email profile',
			response_type: 'token',
			redirect_uri: AUTH_CALLBACK,
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
		window.location = url;
	};
</script>

<svelte:head>
	<script src="https://accounts.google.com/gsi/client" async></script>
</svelte:head>

<button type="button" on:click={onSignIn}>Sign In with Google</button>

<p>
	<a
		href="https://accounts.google.com/o/oauth2/auth?service=lso&o2v=1&ddm=1&flowName=GeneralOAuthFlow"
		>Sign In with Google</a
	>
</p>

<div
	id="g_id_onload"
	data-client_id={AUTH_CLIENT_ID}
	data-context="signin"
	data-ux_mode="redirect"
	data-redirect_uri={AUTH_CALLBACK}
	data-callback="onSignIn"
	data-itp_support="true"
	data-state="HAHA"
	data-locale="en_US"
></div>

<div
	class="g_id_signin"
	data-type="standard"
	data-shape="rectangular"
	data-theme="outline"
	data-text="signin_with"
	data-size="medium"
	data-locale="en-US"
	data-logo_alignment="left"
></div>
