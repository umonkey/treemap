import { getAlert, updateAlert } from '$lib/api/alerts';
import { mapBus } from '$lib/buses/mapBus';
import { goto, routes } from '$lib/routes';
import { locale } from './lang';

export class PageState {
	description = $state<string>('');
	editable = $state<boolean>(false);
	loading = $state<boolean>(true);
	saving = $state<boolean>(false);
	error = $state<string>('');
	currentId = $state<string>('');

	canSave = $derived(this.editable && this.description.trim().length > 0 && !this.saving);

	reload = async (id: string) => {
		if (this.currentId === id) return;

		this.currentId = id;
		this.loading = true;
		this.error = '';

		try {
			const res = await getAlert(id);

			if (res.status === 200 && res.data) {
				if (!res.data.editable) {
					this.editable = false;
					this.description = '';
					this.error = locale.notEditable();
					return;
				}

				this.editable = true;
				this.description = res.data.description ?? '';
			} else {
				this.editable = false;
				this.error = locale.loadFailed();
			}
		} catch (err) {
			console.error('Failed to load the alert.', err);
			this.editable = false;
			this.error = locale.loadFailed();
		} finally {
			this.loading = false;
		}
	};

	handleSave = async () => {
		if (!this.canSave) return;

		this.saving = true;
		this.error = '';

		try {
			const res = await updateAlert(this.currentId, {
				description: this.description.trim()
			});

			if (res.status >= 200 && res.status < 400) {
				mapBus.emit('reload');
				await goto(routes.alertPreview(this.currentId));
			} else {
				console.error(`Error ${res.status} updating the alert.`, res);
				this.error = res.error?.description || locale.saveFailed();
			}
		} catch (err) {
			console.error('Failed to update the alert.', err);
			this.error = locale.saveFailed();
		} finally {
			this.saving = false;
		}
	};

	handleCancel = () => {
		goto(routes.alertPreview(this.currentId));
	};
}
