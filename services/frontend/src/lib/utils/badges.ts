/**
 * Update the application badge (PWA icon count).
 */
export async function updateBadge(count: number) {
	if (typeof navigator !== 'undefined' && 'setAppBadge' in navigator) {
		try {
			if (count > 0) {
				await (navigator as any).setAppBadge(count);
			} else {
				await (navigator as any).clearAppBadge();
			}
		} catch (error) {
			console.error('[badges] Failed to update app badge:', error);
		}
	} else {
		console.debug('[badges] App Badge API is not available.');
	}
}
