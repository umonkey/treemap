<script lang="ts">
	import { googleCallbackHandler } from '$lib/utils/auth';

	window.onSignIn = googleCallbackHandler;

	const getUxMode = (): string => {
		if (window.matchMedia('(display-mode: standalone)').matches) {
			return 'redirect';
		}

		return 'popup';
	};
</script>

<svelte:head>
	<script src="https://accounts.google.com/gsi/client" async></script>
</svelte:head>

<div
	id="g_id_onload"
	data-client_id="999312923392-6k26jala2pe5dk9u7o63o8nvts3a7f1f.apps.googleusercontent.com"
	data-context="signin"
	data-ux_mode={getUxMode()}
	data-callback="onSignIn"
	data-itp_support="true"
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
