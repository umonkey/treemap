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
