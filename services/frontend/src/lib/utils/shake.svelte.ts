import { goto, routes } from '$lib/routes';

/**
 * Shake detector for mobile devices.
 * Uses Device Motion API to detect rapid movement.
 */
class ShakeDetector {
	private threshold = 15; // Acceleration threshold in m/s^2
	private cooldown = 1500; // Cooldown between shakes in ms
	private lastShake = 0;
	private isInitialized = false;

	public isSupported = $state(false);
	public isEnabled = $state(false);

	constructor() {
		if (typeof window !== 'undefined') {
			this.isSupported = 'DeviceMotionEvent' in window;
		}
	}

	/**
	 * Start listening for shake events.
	 * On iOS, this should be preceded by requestPermission().
	 */
	public start = () => {
		if (!this.isSupported || this.isEnabled) return;
		window.addEventListener('devicemotion', this.handleMotion);
		this.isEnabled = true;
		console.debug('[shake] Detector started.');
	};

	/**
	 * Stop listening for shake events.
	 */
	public stop = () => {
		if (!this.isEnabled) return;
		window.removeEventListener('devicemotion', this.handleMotion);
		this.isEnabled = false;
		console.debug('[shake] Detector stopped.');
	};

	/**
	 * Request permission for motion sensors (required for iOS).
	 * Must be called from a user gesture.
	 */
	public requestPermission = async (): Promise<boolean> => {
		if (
			typeof DeviceMotionEvent !== 'undefined' &&
			// eslint-disable-next-line @typescript-eslint/no-explicit-any
			typeof (DeviceMotionEvent as any).requestPermission === 'function'
		) {
			try {
				// eslint-disable-next-line @typescript-eslint/no-explicit-any
				const response = await (DeviceMotionEvent as any).requestPermission();
				const granted = response === 'granted';
				if (granted) {
					this.start();
				}
				return granted;
			} catch (e) {
				console.error('[shake] Permission request failed:', e);
				return false;
			}
		}

		// On other platforms, just start
		this.start();
		return true;
	};

	/**
	 * Initialize the detector on first user interaction.
	 */
	public init = () => {
		if (this.isInitialized) return;

		const handleFirstInteraction = async () => {
			this.isInitialized = true;
			window.removeEventListener('click', handleFirstInteraction);
			window.removeEventListener('touchstart', handleFirstInteraction);
			await this.requestPermission();
		};

		window.addEventListener('click', handleFirstInteraction);
		window.addEventListener('touchstart', handleFirstInteraction);
	};

	private handleMotion = (event: DeviceMotionEvent) => {
		const now = Date.now();
		if (now - this.lastShake < this.cooldown) return;

		const acc = event.acceleration;
		if (!acc) return;

		const { x, y, z } = acc;
		if (x === null || y === null || z === null) return;

		// Calculate total acceleration magnitude
		const totalAcc = Math.sqrt(x * x + y * y + z * z);

		if (totalAcc > this.threshold) {
			this.lastShake = now;
			this.onShake();
		}
	};

	private onShake = () => {
		console.debug('[shake] Shake detected! Going home...');
		goto(routes.home());
	};
}

export const shakeDetector = new ShakeDetector();
