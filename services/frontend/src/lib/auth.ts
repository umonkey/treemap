import { config } from '$lib/env';

export const getSignInUrl = (stateUrl: string): string => {
	const params = new URLSearchParams({
		client_id: config.authClientId,
		scope: 'openid email profile',
		response_type: 'token',
		redirect_uri: config.authCallback,
		response_mode: 'form_post',
		state: stateUrl

		/*
		gsiwebsdk: 'gis_attributes',
		prompt: 'select_account',
		service: 'lso',
		flowName: 'GeneralOAuthFlow',
		*/
	});

	return `https://accounts.google.com/o/oauth2/auth?${params.toString()}`;
};

export const signInWithGoogle = (): void => {
	window.location.assign(getSignInUrl(window.location.href));
};
