import { describe, expect, it } from 'vitest';
import { getSignInUrl } from './auth';
import { config } from '$lib/env';

describe('getSignInUrl', () => {
	it('builds a Google OAuth URL with the expected parameters', () => {
		const url = getSignInUrl('https://example.com/map?x=1');

		expect(url.startsWith('https://accounts.google.com/o/oauth2/auth?')).toBe(true);

		const params = new URLSearchParams(url.split('?')[1]);

		expect(params.get('client_id')).toBe(config.authClientId);
		expect(params.get('redirect_uri')).toBe(config.authCallback);
		expect(params.get('response_type')).toBe('token');
		expect(params.get('response_mode')).toBe('form_post');
		expect(params.get('scope')).toBe('openid email profile');
		expect(params.get('state')).toBe('https://example.com/map?x=1');
	});

	it('round-trips a state URL containing its own query string', () => {
		const stateUrl = 'https://example.com/map?x=1&y=2';
		const url = getSignInUrl(stateUrl);
		const params = new URLSearchParams(url.split('?')[1]);

		expect(params.get('state')).toBe(stateUrl);
	});
});
