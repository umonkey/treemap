import { localize } from '$lib/locale';

interface StringList {
	heading: (id: string) => string;
	tabTitle: (id: string) => string;
	edit: () => string;
	backToAlerts: () => string;
	source: () => string;
	reporter: () => string;
	status: () => string;
	createdAt: () => string;
	respondedAt: () => string;
	coordinates: () => string;
	osm: () => string;
	google: () => string;
	description: () => string;
	response: () => string;
	photos: () => string;
	photoAlt: (id: string) => string;
	loading: () => string;
	web: () => string;
	telegram: () => string;
	unknownReporter: () => string;
	emptyValue: () => string;
	error: (description: string) => string;
}

class English implements StringList {
	heading = (id: string) => `Alert #${id}`;
	tabTitle = (id: string) => `Alert #${id}`;
	edit = () => 'Edit';
	backToAlerts = () => 'Back to alerts';
	source = () => 'Source';
	reporter = () => 'Reporter';
	status = () => 'Status';
	createdAt = () => 'Created at';
	respondedAt = () => 'Responded at';
	coordinates = () => 'Coordinates';
	osm = () => 'OSM';
	google = () => 'Google';
	description = () => 'Description';
	response = () => 'Response';
	photos = () => 'Photos';
	photoAlt = (id: string) => `Photo for alert ${id}`;
	loading = () => 'Loading alert…';
	web = () => 'Web';
	telegram = () => 'Telegram';
	unknownReporter = () => 'Unknown reporter';
	emptyValue = () => '—';
	error = (description: string) => `Error loading alert: ${description}`;
}

class Russian extends English {
	heading = (id: string) => `Сигнал №${id}`;
	tabTitle = (id: string) => `Сигнал №${id}`;
	edit = () => 'Редактировать';
	backToAlerts = () => 'Назад к сигналам';
	source = () => 'Источник';
	reporter = () => 'Автор';
	status = () => 'Статус';
	createdAt = () => 'Создан';
	respondedAt = () => 'Отвечен';
	coordinates = () => 'Координаты';
	osm = () => 'OSM';
	google = () => 'Google';
	description = () => 'Описание';
	response = () => 'Ответ';
	photos = () => 'Фотографии';
	photoAlt = (id: string) => `Фотография к сигналу ${id}`;
	loading = () => 'Загрузка сигнала…';
	web = () => 'Веб-сайт';
	telegram = () => 'Telegram';
	unknownReporter = () => 'Автор неизвестен';
	emptyValue = () => '—';
	error = (description: string) => `Ошибка загрузки сигнала: ${description}`;
}

class Armenian extends English {
	heading = (id: string) => `Ահազանգ #${id}`;
	tabTitle = (id: string) => `Ահազանգ #${id}`;
	edit = () => 'Խմբագրել';
	backToAlerts = () => 'Վերադառնալ ահազանգերին';
	source = () => 'Աղբյուր';
	reporter = () => 'Հեղինակ';
	status = () => 'Կարգավիճակ';
	createdAt = () => 'Ստեղծվել է';
	respondedAt = () => 'Պատասխանվել է';
	coordinates = () => 'Կոորդինատներ';
	osm = () => 'OSM';
	google = () => 'Google';
	description = () => 'Նկարագրություն';
	response = () => 'Պատասխան';
	photos = () => 'Լուսանկարներ';
	photoAlt = (id: string) => `Ահազանգի լուսանկար ${id}`;
	loading = () => 'Ահազանգը բեռնվում է…';
	web = () => 'Վեբ';
	telegram = () => 'Telegram';
	unknownReporter = () => 'Անհայտ հեղինակ';
	emptyValue = () => '—';
	error = (description: string) => `Ահազանգի բեռնման սխալ՝ ${description}`;
}

const getLocale = (): StringList => {
	return localize({
		en: new English(),
		ru: new Russian(),
		hy: new Armenian()
	});
};

export const locale = getLocale();
