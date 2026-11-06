import { localize } from '$lib/locale';

interface StringList {
	title: () => string;
	heading: () => string;
	notice: () => string;
	date: () => string;
	reporter: () => string;
	description: () => string;
	loading: () => string;
	noAlerts: () => string;
	unknownReporter: () => string;
	noDescription: () => string;
	error: (description: string) => string;
}

class English implements StringList {
	title = () => 'Manage Alerts';
	heading = () => 'Manage Alerts';
	notice = () => 'Showing the 100 most recent alerts.';
	date = () => 'Date';
	reporter = () => 'Reporter';
	description = () => 'Description';
	loading = () => 'Loading alerts…';
	noAlerts = () => 'No alerts found.';
	unknownReporter = () => 'Unknown reporter';
	noDescription = () => 'No description';
	error = (description: string) => `Error loading alerts: ${description}`;
}

class Russian extends English {
	title = () => 'Управление сигналами';
	heading = () => 'Управление сигналами';
	notice = () => 'Показаны 100 последних сигналов.';
	date = () => 'Дата';
	reporter = () => 'Автор';
	description = () => 'Описание';
	loading = () => 'Загрузка сигналов…';
	noAlerts = () => 'Сигналы не найдены.';
	unknownReporter = () => 'Автор неизвестен';
	noDescription = () => 'Описание отсутствует';
	error = (description: string) => `Ошибка загрузки сигналов: ${description}`;
}

class Armenian extends English {
	title = () => 'Ահազանգերի կառավարում';
	heading = () => 'Ահազանգերի կառավարում';
	notice = () => 'Ցուցադրված են վերջին 100 ահազանգերը։';
	date = () => 'Ամսաթիվ';
	reporter = () => 'Հեղինակ';
	description = () => 'Նկարագրություն';
	loading = () => 'Ահազանգերը բեռնվում են…';
	noAlerts = () => 'Ահազանգեր չեն գտնվել։';
	unknownReporter = () => 'Անհայտ հեղինակ';
	noDescription = () => 'Նկարագրությունը բացակայում է';
	error = (description: string) => `Ահազանգերի բեռնման սխալ՝ ${description}`;
}

const getLocale = (): StringList => {
	return localize({
		en: new English(),
		ru: new Russian(),
		hy: new Armenian()
	});
};

export const locale = getLocale();
