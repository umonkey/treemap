/**
 * Make sure urls only match report ids, not just random garbage.
 *
 * @param {string} param
 * @returns {boolean}
 */
export function match(param: string) {
	return /^\d+$/.test(param);
}
