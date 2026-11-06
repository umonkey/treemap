export class PhotoPickerLogic {
	handleChange = (event: Event, onFiles: (files: File[]) => void) => {
		const target = event.target as HTMLInputElement;

		if (target.files && target.files.length > 0) {
			onFiles(Array.from(target.files));
		}

		// Reset so the same file can be picked again.
		target.value = '';
	};
}
