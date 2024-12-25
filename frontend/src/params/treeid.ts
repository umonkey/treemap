/**
 * Make sure urls only match tree ids, not just random garbage.
 *
 * @docs https://svelte.dev/docs/kit/advanced-routing#Matching
 * @param {string} param
 * @returns {boolean}
 * @satisfies {import('@sveltejs/kit').ParamMatcher}
 */
export function match(param: string) {
	return /^\d+$/.test(param);
}
