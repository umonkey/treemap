type Item = {
	state: string;
	count: number;
};

type OutItem = {
	label: string;
	value: number;
};

export const formatData = (items: Item[]): OutItem[] => {
	return items.map((item) => ({
		label: item.state,
		value: item.count
	}));
};
