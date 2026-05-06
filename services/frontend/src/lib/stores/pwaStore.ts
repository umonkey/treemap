import { derived, writable } from 'svelte/store';

export interface BeforeInstallPromptEvent extends Event {
	readonly platforms: string[];
	readonly userChoice: Promise<{
		outcome: 'accepted' | 'dismissed';
		platform: string;
	}>;
	prompt(): Promise<void>;
}

declare global {
	interface Window {
		deferredPWAEvent?: BeforeInstallPromptEvent;
	}
}

export const pwaStore = writable<BeforeInstallPromptEvent | undefined>(
	typeof window !== 'undefined' ? window.deferredPWAEvent : undefined
);

export const isInstallable = derived(pwaStore, ($pwaStore) => !!$pwaStore);

export const isManualInstallAvailable = derived(pwaStore, ($pwaStore) => {
	if (typeof window === 'undefined' || typeof navigator === 'undefined') return false;

	// If already installable via native prompt, no need for manual.
	if ($pwaStore) return false;

	// Check if already in standalone mode (installed).
	// @ts-expect-error navigator.standalone is iOS-only
	const isStandalone = window.navigator.standalone || window.matchMedia('(display-mode: standalone)').matches;
	if (isStandalone) return false;

	// Only suggest for mobile devices.
	const isMobile = /iPhone|iPad|iPod|Android/i.test(navigator.userAgent);
	return isMobile;
});
