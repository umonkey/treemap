import { localize } from '$lib/locale';

interface StringList {
	title: () => string;
	descriptionAriaLabel: () => string;
	descriptionPlaceholder: () => string;
	saveButton: () => string;
	savingButton: () => string;
	notEditable: () => string;
	loadFailed: () => string;
	saveFailed: () => string;
}

class English implements StringList {
	title = () => 'Edit alert';
	descriptionAriaLabel = () => 'Description';
	descriptionPlaceholder = () =>
		'Describe what you want to report: tree damage, removal, planting, etc.';
	saveButton = () => 'Save';
	savingButton = () => 'Saving…';
	notEditable = () => 'You are not allowed to edit this alert.';
	loadFailed = () => 'Failed to load the alert. Please try again.';
	saveFailed = () => 'Failed to save the alert. Please try again.';
}

class Russian extends English {
	title = () => 'Редактировать сигнал';
	descriptionAriaLabel = () => 'Описание';
	descriptionPlaceholder = () =>
		'Опишите, о чём вы хотите сообщить: повреждение дерева, вырубка, посадка и т. д.';
	saveButton = () => 'Сохранить';
	savingButton = () => 'Сохранение…';
	notEditable = () => 'У вас нет прав редактировать этот сигнал.';
	loadFailed = () => 'Не удалось загрузить сигнал. Попробуйте ещё раз.';
	saveFailed = () => 'Не удалось сохранить сигнал. Попробуйте ещё раз.';
}

class Armenian extends English {
	title = () => 'Խմբագրել ահազանգը';
	descriptionAriaLabel = () => 'Նկարագրություն';
	descriptionPlaceholder = () =>
		'Նկարագրեք, ինչի մասին ուզում եք հայտնել՝ ծառի վնասվածք, հատում, տնկում և այլն։';
	saveButton = () => 'Պահել';
	savingButton = () => 'Պահվում է…';
	notEditable = () => 'Դուք չունեք այս ահազանգը խմբագրելու թույլտվություն։';
	loadFailed = () => 'Չհաջողվեց բեռնել ահազանգը։ Փորձեք կրկին։';
	saveFailed = () => 'Չհաջողվեց պահել ահազանգը։ Փորձեք կրկին։';
}

const getLocale = (): StringList => {
	return localize({
		en: new English(),
		ru: new Russian(),
		hy: new Armenian()
	});
};

export const locale = getLocale();
