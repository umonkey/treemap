import { derived, writable } from "svelte/store";

interface BeforeInstallPromptEvent extends Event {
	readonly platforms: string[];
	readonly userChoice: Promise<{
		outcome: "accepted" | "dismissed";
		platform: string;
	}>;
	prompt(): Promise<void>;
}

export const pwaStore = writable<BeforeInstallPromptEvent | undefined>(
	undefined,
);
export const isInstallable = derived(pwaStore, ($pwaStore) => !!$pwaStore);
