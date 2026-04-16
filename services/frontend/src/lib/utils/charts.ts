type Item = {
	value: number;
	count: number;
};

type KeyMap = {
	[key: number]: number;
};

/**
 * Normalizes a numeric bar chart data structure by ensuring that all
 * integer values within the range of the minimum and maximum values
 * are represented, even if their count is zero.
 *
 * @param src - An array of items where each item has a numeric value and a count.
 * @returns An array of items with all integer values in the range represented.
 */
export const normalizeNumericBarChart = (src: Item[]): Item[] => {
	const map = src.reduce((acc: KeyMap, item) => {
		acc[item.value] = item.count;
		return acc;
	}, {});

	const keys = Object.keys(map);
	const minKey = Math.min(...keys.map(Number));
	const maxKey = Math.max(...keys.map(Number));

	const res: Item[] = [];

	for (let i = minKey; i <= maxKey; i++) {
		const count = map[i] || 0;
		res.push({ value: i, count });
	}

	return res;
};
