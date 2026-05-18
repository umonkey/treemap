interface NavigatorWithBadge {
	setAppBadge?: (count: number) => Promise<void>;
	clearAppBadge?: () => Promise<void>;
}

/**
 * Update the application badge (PWA icon count).
 */
export async function updateBadge(count: number) {
	const nav = navigator as unknown as NavigatorWithBadge;
	if (nav && typeof nav.setAppBadge !== 'undefined') {
		try {
			if (count > 0 && nav.setAppBadge) {
				await nav.setAppBadge(count);
			} else if (nav.clearAppBadge) {
				await nav.clearAppBadge();
			}
		} catch (error) {
			console.error('[badges] Failed to update app badge:', error);
		}
	} else {
		console.debug('[badges] App Badge API is not available.');
	}
}
