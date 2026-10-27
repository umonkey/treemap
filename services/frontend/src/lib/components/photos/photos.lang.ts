import { localize } from '$lib/locale';

interface StringList {
	takePhoto: () => string;
	pickFromGallery: () => string;
}

class English implements StringList {
	takePhoto = () => 'Take a photo';
	pickFromGallery = () => 'Pick from gallery';
}

class Russian extends English {
	takePhoto = () => 'Сделать фото';
	pickFromGallery = () => 'Выбрать из галереи';
}

class Armenian extends English {
	takePhoto = () => 'Լուսանկարել';
	pickFromGallery = () => 'Ընտրել պատկերասրահից';
}

const getLocale = (): StringList => {
	return localize({
		en: new English(),
		ru: new Russian(),
		hy: new Armenian()
	});
};

export const locale = getLocale();
