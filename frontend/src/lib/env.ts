export const API_ROOT = import.meta.env.VITE_API_ROOT ?? 'https://yerevan.treemaps.app/';
export const AUTH_CALLBACK = `${API_ROOT}v3/login/google`;
export const AUTH_CLIENT_ID =
	import.meta.env.VITE_AUTH_CLIENT_ID ??
	'999312923392-6k26jala2pe5dk9u7o63o8nvts3a7f1f.apps.googleusercontent.com';

export const GTM_ID = import.meta.env.VITE_GTM_ID ?? 'G-CEJ6L6RMWG';
export const ENVIRONMENT = import.meta.env.VITE_ENVIRONMENT ?? 'development';

export const OSM_AUTH_CLIENT_ID = 'mCL7JIK8ky2_7g6vC9t2cI-jUh1nsA1nK5bQ2AA2VK8';
