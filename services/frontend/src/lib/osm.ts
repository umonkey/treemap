import { config } from '$lib/env';

export const getAuthenticateUrl = (origin: string): string => {
	const params = new URLSearchParams({
		client_id: config.osmAuthClientId,
		redirect_uri: `${origin}/auth/osm/callback`.replace('http:', 'https:'),
		response_type: 'code',
		scope: 'write_api write_redactions',
		state: 'none'
	});

	return `https://www.openstreetmap.org/oauth2/authorize?${params.toString()}`;
};
