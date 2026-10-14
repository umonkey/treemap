import type { Component } from 'svelte';
import ChartIcon from '$lib/icons/ChartIcon.svelte';
import DatabaseIcon from '$lib/icons/DatabaseIcon.svelte';
import LearnIcon from '$lib/icons/LearnIcon.svelte';
import MergeIcon from '$lib/icons/MergeIcon.svelte';
import QuestionIcon from '$lib/icons/QuestionIcon.svelte';
import StreetIcon from '$lib/icons/StreetIcon.svelte';
import StreetViewIcon from '$lib/icons/StreetViewIcon.svelte';
import TriangleIcon from '$lib/icons/TriangleIcon.svelte';
import UsersIcon from '$lib/icons/UsersIcon.svelte';
import { locale } from './tools.lang';

export type Tool = {
	name: string;
	description: string;
	href: string;
	icon: Component;
	permission?: string;
};

export const tools: Tool[] = [
	{
		name: locale.learn(),
		description: locale.learnDescription(),
		href: '/tools/learn',
		icon: LearnIcon
	},
	{
		name: locale.trilaterate(),
		description: locale.trilaterateDescription(),
		href: '/tools/range',
		icon: TriangleIcon,
		permission: 'tree:create'
	},
	{
		name: locale.duplicates(),
		description: locale.duplicatesDescription(),
		href: '/tools/merge',
		icon: MergeIcon,
		permission: 'tree:delete'
	},
	{
		name: locale.guessSpecies(),
		description: locale.guessSpeciesDescription(),
		href: '/tools/unknown-species',
		icon: QuestionIcon,
		permission: 'tree:create'
	},
	{
		name: locale.statistics(),
		description: locale.statisticsDescription(),
		href: '/tools/stats',
		icon: ChartIcon
	},
	{
		name: locale.reports(),
		description: locale.reportsDescription(),
		href: '/tools/street-report',
		icon: StreetIcon
	},
	{
		name: locale.panoramas(),
		description: locale.panoramasDescription(),
		href: '/tools/panoramas',
		icon: StreetViewIcon,
		permission: 'pano:edit'
	},
	{
		name: locale.users(),
		description: locale.usersDescription(),
		href: '/tools/users',
		icon: UsersIcon,
		permission: 'user:manage'
	},
	{
		name: locale.exportData(),
		description: locale.exportDataDescription(),
		href: '/tools/export',
		icon: DatabaseIcon
	}
];

export const isActiveTool = (href: string, pathname: string): boolean =>
	pathname === href || pathname.startsWith(`${href}/`);
