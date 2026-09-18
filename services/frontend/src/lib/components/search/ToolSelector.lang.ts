import { localize } from '$lib/locale';

interface StringList {
	operationMode: () => string;
	toolShow: () => string;
	toolHeight: () => string;
	toolDiameter: () => string;
	toolCircumference: () => string;
	toolChecklist: () => string;
	toolCamera: () => string;
	hintShow: () => string;
	hintHeight: () => string;
	hintDiameter: () => string;
	hintCircumference: () => string;
	hintChecklist: () => string;
	hintCamera: () => string;
}

class English implements StringList {
	operationMode = () => 'Select operation mode';
	toolShow = () => 'Show';
	toolHeight = () => 'Height';
	toolDiameter = () => 'Diameter';
	toolCircumference = () => 'Circumference';
	toolChecklist = () => 'Checklist';
	toolCamera = () => 'Camera';
	hintShow = () => 'General tree inspection';
	hintHeight = () => 'Measure height (have a laser)';
	hintDiameter = () => 'Measure crown diameter (have a laser)';
	hintCircumference = () => 'Measure circumference (have tape measure)';
	hintChecklist = () => 'Update defects and other conditions';
	hintCamera = () => 'Take fresh photos';
}

class Russian extends English {
	operationMode = () => 'Выберите режим работы';
	toolShow = () => 'Просмотр';
	toolHeight = () => 'Высота';
	toolDiameter = () => 'Диаметр';
	toolCircumference = () => 'Обхват';
	toolChecklist = () => 'Чек-лист';
	toolCamera = () => 'Камера';
	hintShow = () => 'Общий осмотр деревьев';
	hintHeight = () => 'Измерение высоты (есть дальномер)';
	hintDiameter = () => 'Измерение диаметра кроны (есть дальномер)';
	hintCircumference = () => 'Измерение обхвата ствола (есть рулетка)';
	hintChecklist = () => 'Определение дефектов и особенностей';
	hintCamera = () => 'Добавление свежих фотографий (есть камера)';
}

class Armenian extends English {
	operationMode = () => 'Ընտրեք աշխատանքի ռեժիմը';
	toolShow = () => 'Դիտում';
	toolHeight = () => 'Բարձրություն';
	toolDiameter = () => 'Տրամագիծ';
	toolCircumference = () => 'Շրջագիծ';
	toolChecklist = () => 'Ստուգաթերթ';
	toolCamera = () => 'Տեսախցիկ';
	hintShow = () => 'Ծառերի ընդհանուր զննում';
	hintHeight = () => 'Բարձրության չափում (կա հեռաչափ)';
	hintDiameter = () => 'Հովանու տրամագծի չափում (կա հեռաչափ)';
	hintCircumference = () => 'Բնի շրջագծի չափում (կա չափիչ ժապավեն)';
	hintChecklist = () => 'Արատների և առանձնահատկությունների որոշում';
	hintCamera = () => 'Թարմ լուսանկարների ավելացում (կա տեսախցիկ)';
}

const getLocale = (): StringList => {
	return localize({
		en: new English(),
		ru: new Russian(),
		hy: new Armenian()
	});
};

export const locale = getLocale();
