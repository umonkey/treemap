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
