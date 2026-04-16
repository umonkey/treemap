export type Record = {
	species: string;
	count: number;
	height: number;
	diameter: number;
	girth: number;
};

class ReportState {
	records = $state<Record[]>([]);
	street = $state<string>('');
	total = $state<number>(0);

	public reload(street: string, total: number, records: Record[]): void {
		this.street = street;
		this.total = total;
		this.records = records;
	}

	sortSpecies = (): void => {
		this.sort('species', 1);
	};

	sortCount = (): void => {
		this.sort('count');
	};

	sortHeight = (): void => {
		this.sort('height');
	};

	sortCrown = (): void => {
		this.sort('diameter');
	};

	sortGirth = (): void => {
		this.sort('girth');
	};

	sort = (key: keyof Record, order = -1): void => {
		const records = [...this.records];

		// First sort by name, as if we later sort by e.g. girth,
		// and there are many items with 0 avg girth, they would
		// be sorted semi-randomly upon every click.  This makes
		// the list predictable by eliminating the randomness.
		records.sort((a, b) => {
			return order * (a.species > b.species ? 1 : -1);
		});

		if (key !== 'species') {
			records.sort((a, b) => {
				return order * (a[key] > b[key] ? 1 : -1);
			});
		}

		this.records = records;
	};
}

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

export const reportState = new ReportState();
