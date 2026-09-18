import { localize } from '$lib/locale';

interface StringList {
	baseDetails: () => string;
	baseLight: () => string;
	baseSatellite: () => string;
	layerDrone: () => string;
	layerAlerts: () => string;
	layerPanoramas: () => string;
	layerTreeHints: () => string;
	layerWater: () => string;
	optionStickyPoints: () => string;
	optionCenter: () => string;
}

class English implements StringList {
	baseDetails = () => 'Details';
	baseLight = () => 'Light';
	baseSatellite = () => 'Satellite';
	layerDrone = () => 'Drone imagery';
	layerAlerts = () => 'Alerts';
	layerPanoramas = () => 'Panoramas';
	layerTreeHints = () => 'Tree hints';
	layerWater = () => 'Water sources';
	optionStickyPoints = () => 'Sticky points';
	optionCenter = () => 'Center crosshair';
}

class Russian extends English {
	baseDetails = () => 'Детальный';
	baseLight = () => 'Светлый';
	baseSatellite = () => 'Спутник';
	layerDrone = () => 'Снимки с дрона';
	layerAlerts = () => 'Сигналы';
	layerPanoramas = () => 'Панорамы';
	layerTreeHints = () => 'Подсказки по деревьям';
	layerWater = () => 'Источники воды';
	optionStickyPoints = () => 'Прилипание к точкам';
	optionCenter = () => 'Перекрестие в центре';
}

class Armenian extends English {
	baseDetails = () => 'Մանրամասն';
	baseLight = () => 'Լուսավոր';
	baseSatellite = () => 'Արբանյակ';
	layerDrone = () => 'Անօդաչուի լուսանկարներ';
	layerAlerts = () => 'Ահազանգեր';
	layerPanoramas = () => 'Պանորամաներ';
	layerTreeHints = () => 'Ծառերի հուշումներ';
	layerWater = () => 'Ջրի աղբյուրներ';
	optionStickyPoints = () => 'Կպչուն կետեր';
	optionCenter = () => 'Կենտրոնական խաչ';
}

const getLocale = (): StringList => {
	return localize({
		en: new English(),
		ru: new Russian(),
		hy: new Armenian()
	});
};

export const locale = getLocale();
