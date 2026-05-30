/**
 * An object oriented wrapper for localStorage.
 */

class LocalStorage {
	private isAvailable(): boolean {
		return typeof window !== 'undefined' && typeof localStorage !== 'undefined';
	}

	public read<T>(key: string): T | undefined {
		if (!this.isAvailable()) {
			return undefined;
		}

		try {
			const value = localStorage.getItem(key);

			if (value === null) {
				return undefined;
			}

			// console.debug(`[storage] Read ${key} = ${value}`);
			return JSON.parse(value);
		} catch (e) {
			console.error(`[storage] Error reading ${key}`, e);
		}
	}

	public write<T>(key: string, value: T | null): boolean {
		if (!this.isAvailable()) {
			return false;
		}

		try {
			if (value === undefined || value === null) {
				localStorage.removeItem(key);
				// console.debug(`[storage] Removed ${key}`);
			} else {
				const data = JSON.stringify(value);
				localStorage.setItem(key, data);
				// console.debug(`[storage] Wrote ${key} = ${data}`);
			}
			return true;
		} catch (e) {
			console.error(`[storage] Error writing ${key}`, e);
			return false;
		}
	}
}

export const ls = new LocalStorage();
