// Code that generates a heat map based on daily update counts.
// Uses log() to scale the values for better visualization.

import { type IHeatMap } from '$lib/types';

export type Cell = {
	date: string;
	value: string;
	grade: number;
	title: string;
};

export class HeatMap {
	private getMinDate = (values: IHeatMap[]): Date => {
		const dates = values.map((v) => v.date);
		dates.sort();
		return new Date(dates[0]);
	};

	private getMaxDate = (values: IHeatMap[]): Date => {
		const dates = values.map((v) => v.date);
		dates.sort();
		return new Date(dates[dates.length - 1]);
	};

	private getWeekStart = (date: Date): Date => {
		const day = date.getDay();
		const diff = date.getDate() - (day === 0 ? 6 : day - 1);
		return new Date(date.setDate(diff));
	};

	private getNextWeek = (date: Date): Date => {
		const week = this.getWeekStart(date);
		week.setDate(week.getDate() + 7);
		return week;
	};

	private getGrade = (value: number, maxScale: number): number => {
		if (!value) {
			return 0;
		}

		const grade = (Math.log(value + 1) * 3) / maxScale;
		return Math.floor(grade) + 1;
	};

	private getInputValue = (values: IHeatMap[], date: Date, maxScale: number): Cell => {
		const d = date.toISOString().split('T')[0];

		for (const value of values) {
			if (value.date === d) {
				return {
					date: d,
					value: value.value.toString(),
					grade: this.getGrade(value.value, maxScale),
					title: `${value.value} trees updated on ${d}`
				};
			}
		}

		return {
			date: d,
			value: '-',
			grade: 0,
			title: `No updates for ${d}`
		};
	};

	private getMaxScale = (items: IHeatMap[]): number => {
		const values = items.map((v) => Math.log(v.value + 1));
		return Math.max(...values);
	};

	public formatData = (values: IHeatMap[]): Cell[][] => {
		const firstDate = this.getWeekStart(this.getMinDate(values));
		const lastDate = this.getNextWeek(this.getMaxDate(values));
		const maxScale = this.getMaxScale(values);

		const rows: Cell[][] = [[], [], [], [], [], [], []];

		for (let row = 0; row < 7; row++) {
			const currentDate = new Date(firstDate);
			currentDate.setDate(currentDate.getDate() + row);

			while (currentDate < lastDate) {
				const value = this.getInputValue(values, currentDate, maxScale);
				rows[row].push(value);
				currentDate.setDate(currentDate.getDate() + 7);
			}
		}

		return rows;
	};
}
