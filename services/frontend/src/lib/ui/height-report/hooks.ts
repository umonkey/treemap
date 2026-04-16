import { normalizeNumericBarChart } from '$lib/utils/charts';

type Item = {
	value: number;
	count: number;
};

type Res = {
	label: string;
	value: number;
};

export const formatData = (items: Item[]): Res[] => {
	const normalized = normalizeNumericBarChart(items);

	const res: Res[] = normalized.map((item) => ({
		label: `${item.value} m`,
		value: item.count
	}));

	return res;
};
