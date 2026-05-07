import { page } from '$app/state';
import { locale } from '$lib/locale';
import { routes } from '$lib/routes';

class TabsLogic {
	tabs = $derived.by(() => {
		const path = page.url.pathname;

		return [
			{
				id: 'trees',
				title: locale.tabTrees(),
				link: routes.treeUpdates(),
				active: path === routes.treeUpdates()
			},
			{
				id: 'comments',
				title: locale.updatesComments(),
				link: routes.comments(),
				active: path === routes.comments()
			}
		];
	});
}

export const componentState = new TabsLogic();
