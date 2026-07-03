import { getUser } from '$lib/stores/userStore';
import type { ITreeFile } from '$lib/types';
import { get } from 'svelte/store';

export const formatDate = (timestamp: number): string => {
	const date = new Date(timestamp * 1000);

	const day = date.getDate().toString().padStart(2, '0');
	const month = (date.getMonth() + 1).toString().padStart(2, '0');
	const year = date.getFullYear();

	return `${day}.${month}.${year}`;
};

export const formatDateTime = (timestamp: number): string => {
	const date = new Date(timestamp * 1000);

	const day = date.getDate().toString().padStart(2, '0');
	const month = (date.getMonth() + 1).toString().padStart(2, '0');
	const year = date.getFullYear();

	const hours = date.getHours().toString().padStart(2, '0');
	const minutes = date.getMinutes().toString().padStart(2, '0');

	return `${day}.${month}.${year} ${hours}:${minutes}`;
};

export const formatDateTimeISO = (timestamp: number): string => {
	const date = new Date(timestamp * 1000);

	const day = date.getDate().toString().padStart(2, '0');
	const month = (date.getMonth() + 1).toString().padStart(2, '0');
	const year = date.getFullYear();

	const hours = date.getHours().toString().padStart(2, '0');
	const minutes = date.getMinutes().toString().padStart(2, '0');

	return `${year}-${month}-${day} ${hours}:${minutes}`;
};

export const fileAttribution = (file: ITreeFile): string => {
	if (!file.added_at || !file.added_by) {
		return '';
	}

	const user = get(getUser)(file.added_by);

	if (user === undefined) {
		return '';
	}

	const date = formatDate(file.added_at);
	return `${date} by ${user.name}`;
};

// Round to 7 decimal places, which is 1 cm accuracy in Yerevan.
export const roundCoord = (value: number): number => {
	return Math.round(value * 10000000) / 10000000;
};

export const formatSize = (bytes: number): string => {
	if (bytes === 0) return '0 B';
	const k = 1024;
	const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
	const i = Math.floor(Math.log(bytes) / Math.log(k));
	return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
};
