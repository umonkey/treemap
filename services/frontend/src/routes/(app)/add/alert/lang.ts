import { localize } from '$lib/locale';

interface StringList {
	title: () => string;
	descriptionAriaLabel: () => string;
	descriptionPlaceholder: () => string;
	sendButton: () => string;
	sendingButton: () => string;
	takePhoto: () => string;
	pickFromGallery: () => string;
	removePhoto: () => string;
	photoAlt: () => string;
	tooManyPhotos: (count: number) => string;
	uploadFailed: () => string;
}

class English implements StringList {
	title = () => 'Add alert';
	descriptionAriaLabel = () => 'Description';
	descriptionPlaceholder = () =>
		'Describe what you want to report: tree damage, removal, planting, etc.';
	sendButton = () => 'Send';
	sendingButton = () => 'Sending…';
	takePhoto = () => 'Take a photo';
	pickFromGallery = () => 'Pick from gallery';
	removePhoto = () => 'Remove photo';
	photoAlt = () => 'Queued alert photo';
	tooManyPhotos = (count: number) => `You can attach at most ${count} photos.`;
	uploadFailed = () => 'Failed to send the alert. Please try again.';
}

class Russian extends English {
	title = () => 'Добавить сигнал';
	descriptionAriaLabel = () => 'Описание';
	descriptionPlaceholder = () =>
		'Опишите, о чём вы хотите сообщить: повреждение дерева, вырубка, посадка и т. д.';
	sendButton = () => 'Отправить';
	sendingButton = () => 'Отправка…';
	takePhoto = () => 'Сделать фото';
	pickFromGallery = () => 'Выбрать из галереи';
	removePhoto = () => 'Удалить фото';
	photoAlt = () => 'Фото сигнала в очереди';
	tooManyPhotos = (count: number) => `Можно прикрепить не более ${count} фотографий.`;
	uploadFailed = () => 'Не удалось отправить сигнал. Попробуйте ещё раз.';
}

class Armenian extends English {
	title = () => 'Ավելացնել ահազանգ';
	descriptionAriaLabel = () => 'Նկարագրություն';
	descriptionPlaceholder = () =>
		'Նկարագրեք, ինչի մասին ուզում եք հայտնել՝ ծառի վնասվածք, հատում, տնկում և այլն։';
	sendButton = () => 'Ուղարկել';
	sendingButton = () => 'Ուղարկվում է…';
	takePhoto = () => 'Լուսանկարել';
	pickFromGallery = () => 'Ընտրել պատկերասրահից';
	removePhoto = () => 'Հեռացնել լուսանկարը';
	photoAlt = () => 'Հերթում գտնվող ահազանգի լուսանկար';
	tooManyPhotos = (count: number) => `Կարելի է կցել առավելագույնը ${count} լուսանկար։`;
	uploadFailed = () => 'Չհաջողվեց ուղարկել ահազանգը։ Փորձեք կրկին։';
}

const getLocale = (): StringList => {
	return localize({
		en: new English(),
		ru: new Russian(),
		hy: new Armenian()
	});
};

export const locale = getLocale();
