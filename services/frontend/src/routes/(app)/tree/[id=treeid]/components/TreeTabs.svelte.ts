import { locale } from '$lib/locale';
import { routes } from '$lib/routes';

export interface ITreeTab {
	id: string;
	title: string;
	url: string;
	/** Relative path appended to a tree URL to stay on this tab. */
	suffix: string;
}

export const treeTabs = (id: string): ITreeTab[] => [
	{ id: 'preview', title: locale.treeTabsPreview(), url: routes.treeDetails(id), suffix: '' },
	{ id: 'photos', title: locale.treeTabsPhotos(), url: routes.treePhotos(id), suffix: '/photos' },
	{
		id: 'comments',
		title: locale.treeTabsComments(),
		url: routes.treeComments(id),
		suffix: '/comments'
	},
	{
		id: 'observations',
		title: locale.treeTabsObservations(),
		url: routes.treeObservations(id),
		suffix: '/observations'
	},
	{
		id: 'history',
		title: locale.treeTabsChanges(),
		url: routes.treeHistory(id),
		suffix: '/history'
	}
];

/** The tab the current URL belongs to, or undefined for a non-tab sub-page. */
export const activeTreeTab = (pathname: string, id: string): ITreeTab | undefined =>
	treeTabs(id).find((tab) => tab.url === pathname);
