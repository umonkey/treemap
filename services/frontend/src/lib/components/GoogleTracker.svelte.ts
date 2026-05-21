import { browser } from '$app/environment';
import { config } from '$lib/env';

class GoogleTrackerLogic {
	gaId = config.gaMeasurementId;
	private lastUserId: string | undefined = undefined;

	init = (userId?: string) => {
		if (!browser || !this.gaId) return;

		window.dataLayer = window.dataLayer || [];
		window.gtag = function () {
			// eslint-disable-next-line prefer-rest-params
			window.dataLayer.push(arguments);
		};
		window.gtag('js', new Date());

		console.debug('[GA] Initializing for ID:', this.gaId);

		const gtagConfig: Record<string, string | boolean> = {
			send_page_view: false
		};

		if (userId) {
			gtagConfig.user_id = userId;
			this.lastUserId = userId;
		}

		if (window.location.hostname === 'localhost' || window.location.hostname === '127.0.0.1') {
			gtagConfig.debug_mode = true;
			gtagConfig.cookie_domain = 'none';
		}

		window.gtag('config', this.gaId, gtagConfig);
	};

	trackPageView = (url: string, path: string, title: string) => {
		if (!browser || !this.gaId || typeof window.gtag !== 'function') return;

		console.debug('[GA] Tracking page view:', path);

		window.gtag('event', 'page_view', {
			page_title: title,
			page_location: url,
			page_path: path
		});
	};

	setUserId = (userId: string | undefined) => {
		if (!browser || !this.gaId || typeof window.gtag !== 'function' || !userId) return;

		if (userId === this.lastUserId) return;

		console.debug('[GA] Setting user ID:', userId);
		window.gtag('set', { user_id: userId });
		this.lastUserId = userId;
	};
}

export const componentState = new GoogleTrackerLogic();
