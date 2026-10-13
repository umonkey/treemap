/**
 * Device utility functions for detecting constrained environments.
 */

/**
 * Check if the user has requested data-saving mode.
 * Supported in Chromium-based browsers via the Network Information API.
 */
export const isDataSaving = (): boolean => {
	if (typeof navigator === 'undefined') {
		return false;
	}

	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	const conn = (navigator as any).connection;
	return !!conn?.saveData;
};

/**
 * Check if the device is in a low-power state.
 * This is a heuristic based on the Battery Status API.
 * Returns true if the battery is not charging and level is 20% or lower.
 */
export const isLowPower = async (): Promise<boolean> => {
	if (typeof navigator === 'undefined' || !('getBattery' in navigator)) {
		return false;
	}

	try {
		// eslint-disable-next-line @typescript-eslint/no-explicit-any
		const battery = await (navigator as any).getBattery();
		return !battery.charging && battery.level <= 0.2;
	} catch (e) {
		console.debug('[device] Failed to get battery status:', e);
		return false;
	}
};

/**
 * Check if the device's primary input is a coarse pointer (touch), i.e. a
 * phone or tablet. On such devices the OS picker offers a distinct photo
 * library ("gallery"); on desktop the camera and gallery tiles would open the
 * same file dialog, so only the camera tile is shown there.
 */
export const isTouchDevice = (): boolean => {
	if (typeof window === 'undefined') {
		return false;
	}

	return window.matchMedia('(hover: none) and (pointer: coarse)').matches;
};
