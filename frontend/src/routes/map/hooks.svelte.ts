import { mapBus } from '$lib/buses/mapBus';
import { goto, routes } from '$lib/routes';

class PageState {
	public handleTreeChange = (id: string | undefined | null) => {
		// When the tree id changes in page arguments,
		// trigger the preview pane to be shown.
		if (id) {
			mapBus.emit('preview', id);
		}
	};

	// Monitor for tree selection.  When a new tree is clicked,
	// change the page address to show the right preview.
	public onMount = () => {
		const handler = (id: string) => {
			goto(routes.mapPreview(id));
		};

		mapBus.on('select', handler);

		return () => {
			mapBus.off('select', handler);
		};
	};
}

export const pageState = new PageState();
