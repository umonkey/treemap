/**
 * An object oriented wrapper for localStorage.
 */

class LocalStorage {
	public read<T>(key: string): T | undefined {
		try {
			const value = localStorage.getItem(key);

			if (value === null) {
				return undefined;
			}

			console.debug(`[storage] Read ${key}`);
			return JSON.parse(value);
		} catch (e) {
			console.error(`[storage] Error reading ${key}: ${e}`);
		}
	}

	public write<T>(key: string, value: T | null): void {
		try {
			if (value === undefined || value === null) {
				localStorage.removeItem(key);
				console.debug(`[storage] Removed ${key}`);
			} else {
				localStorage.setItem(key, JSON.stringify(value));
				console.debug(`[storage] Wrote ${key}`);
			}
		} catch (e) {
			console.error(`[storage] Error writing ${key}: ${e}`);
		}
	}
}

export const ls = new LocalStorage();
