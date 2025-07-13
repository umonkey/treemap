type Item = {
	species: string;
	count: number;
};

type Res = {
	label: string;
	value: number;
};

export const formatData = (items: Item[]): Res[] => {
	return items.map((item) => ({
		label: item.species,
		value: item.count
	}));
};

export const formatNumber = (num: number): string => {
	return num.toFixed(1);
};

export const pc = (num: number, total: number): string => {
	if (total <= 0) {
		return '0%';
	}

	const pc = Math.round((num / total) * 100);
	return `${pc}%`;
};
