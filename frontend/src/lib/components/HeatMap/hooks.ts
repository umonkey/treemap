import { type IHeatMap } from '$lib/types';

type InputMap = {
	[key: string]: IHeatMap[];
};

type Cell = {
	date: string;
	value: string;
	grade: number;
	title: string;
};

const getMinDate = (values: IHeatMap[]): Date => {
	const dates = values.map(v => v.date);
	dates.sort();
	return new Date(dates[0]);
};

const getMaxDate = (values: IHeatMap[]): Date => {
	const dates = values.map(v => v.date);
	dates.sort();
	return new Date(dates[dates.length - 1]);
};

const getWeekStart = (date: Date): Date => {
	const day = date.getDay();
	const diff = date.getDate() - (day === 0 ? 6 : day - 1);
	return new Date(date.setDate(diff));
};

const getNextWeek = (date: Date): Date => {
	const week = getWeekStart(date);
	week.setDate(week.getDate() + 7);
	return week;
}

const getGrade = (value: number, maxValue: number): number => {
	const grade = value * 5 / maxValue;
	return Math.floor(grade);
};

const getInputValue = (values: IHeatMap[], date: Date, maxValue: number): Cell => {
	const d = date.toISOString().split('T')[0];

	for (const value of values) {
		if (value.date === d) {
			return {
				date: d,
				value: value.value.toString(),
				grade: getGrade(value.value, maxValue),
				title: `${value.value} updates on ${d}`,
			};
		}
	}

	return {
		date: d,
		value: '-',
		grade: 0,
		title: `No data for ${d}`,
	};
};

const getMaxValue = (items: IHeatMap[]): number => {
	let values = items.map(v => v.value);
	return Math.max(...values);
}

export const getRandomData = (): IHeatMap[] => {
	const data = [];

	let currentDate = new Date('2024-08-04');

	while (data.length < 365) {
		const d = currentDate.toISOString().split('T')[0];
		const v = Math.floor(Math.random() * 100);

		data.push({
			date: d,
			value: v,
		});

		currentDate.setDate(currentDate.getDate() + 1);
	}

	return data;
};

export const formatData = (values: IHeatMap[]): Cell[][] => {
	const firstDate = getWeekStart(getMinDate(values));
	const lastDate = getNextWeek(getMaxDate(values));
	const maxValue = getMaxValue(values);

	const rows: Cell[][] = [[], [], [], [], [], [], []];

	for (let row = 0; row < 7; row++) {
		const currentDate = new Date(firstDate);
		currentDate.setDate(currentDate.getDate() + row);

		while (currentDate < lastDate) {
			const value = getInputValue(values, currentDate, maxValue);
			rows[row].push(value);
			currentDate.setDate(currentDate.getDate() + 7);
		};
	}

	return rows;
};
