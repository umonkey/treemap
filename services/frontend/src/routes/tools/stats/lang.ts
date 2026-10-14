import { localize } from '$lib/locale';

interface StringList {
	title: () => string;
	header: () => string;
	heatmapHeader: () => string;
	contributorsHeader: () => string;
	reportsHeader: () => string;
	species: () => string;
	streets: () => string;
	height: () => string;
	diameter: () => string;
	circumference: () => string;
	state: () => string;
	streetReport: () => string;
}

class English implements StringList {
	title = () => 'Stats and Analytics';
	header = () => 'Stats and Analytics';
	heatmapHeader = () => 'Global User Activity';
	contributorsHeader = () => 'Top contributors';
	reportsHeader = () => 'Reports';
	species = () => 'Top species';
	streets = () => 'Top streets';
	height = () => 'Top by height';
	diameter = () => 'Top by diameter';
	circumference = () => 'Top by trunk circumference';
	state = () => 'Group by state';
	streetReport = () => 'Street report';
}

class Russian extends English {
	title = () => 'Анализ данных';
	header = () => 'Анализ данных';
	heatmapHeader = () => 'Общая активность';
	contributorsHeader = () => 'Лучшие участники';
	reportsHeader = () => 'Отчёты';
	species = () => 'Топ видов';
	streets = () => 'Топ улиц';
	height = () => 'Самые высокие';
	diameter = () => 'Самые широкие';
	circumference = () => 'Топ по обхвату ствола';
	state = () => 'Группировка по состоянию';
	streetReport = () => 'Отчёт по улице';
}

class Armenian extends English {
	title = () => 'Տվյալների զեկույցներ';
	header = () => 'Տվյալների զեկույցներ';
	heatmapHeader = () => 'Օգտվողների ընդհանուր ակտիվությունը';
	contributorsHeader = () => 'Լավագույն ներդրողներ';
	reportsHeader = () => 'Հաշվետվություններ';
	species = () => 'Լավագույն տեսակներ';
	streets = () => 'Լավագույն փողոցներ';
	height = () => 'Ամենաբարձր ծառերը';
	diameter = () => 'Ամենալայն ծառերը';
	circumference = () => 'Ըստ բնի շրջագծի';
	state = () => 'Խմբավորել ըստ վիճակի';
	streetReport = () => 'Փողոցի զեկույց';
}

const getLocale = (): StringList => {
	return localize({
		en: new English(),
		ru: new Russian(),
		hy: new Armenian()
	});
};

export const locale = getLocale();
