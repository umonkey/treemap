import { localize } from '$lib/locale';

interface StringList {
	title: () => string;
	baseDetails: () => string;
	baseLight: () => string;
	baseSatellite: () => string;
	additionalTitle: () => string;
	layerDrone: () => string;
	layerAlerts: () => string;
	layerPanoramas: () => string;
	otherTitle: () => string;
	optionStickyPoints: () => string;
}

class English implements StringList {
	title = () => 'Select map base layer';
	baseDetails = () => 'Details';
	baseLight = () => 'Light';
	baseSatellite = () => 'Satellite';
	additionalTitle = () => 'Select additional layers';
	layerDrone = () => 'Drone imagery';
	layerAlerts = () => 'Alerts';
	layerPanoramas = () => 'Panoramas';
	otherTitle = () => 'Other options';
	optionStickyPoints = () => 'Sticky points';
}

class Russian extends English {
	title = () => 'Выберите базовый слой';
	baseDetails = () => 'Детальный';
	baseLight = () => 'Светлый';
	baseSatellite = () => 'Спутник';
	additionalTitle = () => 'Дополнительные слои';
	layerDrone = () => 'Снимки с дрона';
	layerAlerts = () => 'Сигналы';
	layerPanoramas = () => 'Панорамы';
	otherTitle = () => 'Другие настройки';
	optionStickyPoints = () => 'Прилипание к точкам';
}

class Armenian extends English {
	title = () => 'Ընտրել հիմնական շերտը';
	baseDetails = () => 'Մանրամասն';
	baseLight = () => 'Լուսավոր';
	baseSatellite = () => 'Արբանյակ';
	additionalTitle = () => 'Ընտրել լրացուցիչ շերտեր';
	layerDrone = () => 'Անօդաչուի լուսանկարներ';
	layerAlerts = () => 'Ահազանգեր';
	layerPanoramas = () => 'Պանորամաներ';
	otherTitle = () => 'Այլ կարգավորումներ';
	optionStickyPoints = () => 'Կպչուն կետեր';
}

const getLocale = (): StringList => {
	return localize({
		en: new English(),
		ru: new Russian(),
		hy: new Armenian()
	});
};

export const locale = getLocale();
