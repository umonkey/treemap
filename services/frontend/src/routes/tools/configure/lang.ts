import { localize } from '$lib/locale';

interface StringList {
	title: () => string;
	name: () => string;
	description: () => string;
	osmIntegration: () => string;
	osmPull: () => string;
	osmBotId: () => string;
	osmBotIdHint: () => string;
	apply: () => string;
	applying: () => string;
	loading: () => string;
	error: (description: string) => string;
}

class English implements StringList {
	title = () => 'Configure';
	name = () => 'Name';
	description = () => 'Description';
	osmIntegration = () => 'OSM Integration';
	osmPull = () => 'Pull new trees from OSM';
	osmBotId = () => 'OSM bot user ID';
	osmBotIdHint = () => 'Local user id used for changes pulled from OSM.';
	apply = () => 'Apply';
	applying = () => 'Applying...';
	loading = () => 'Loading...';
	error = (description: string) => `Error: ${description}`;
}

class Russian extends English {
	title = () => 'Настройки';
	name = () => 'Название';
	description = () => 'Описание';
	osmIntegration = () => 'Интеграция с OSM';
	osmPull = () => 'Импортировать деревья из OSM';
	osmBotId = () => 'ID пользователя бота OSM';
	osmBotIdHint = () => 'ID локального пользователя для изменений из OSM.';
	apply = () => 'Применить';
	applying = () => 'Сохранение...';
	loading = () => 'Загрузка...';
	error = (description: string) => `Ошибка: ${description}`;
}

class Armenian extends English {
	title = () => 'Կարգավորումներ';
	name = () => 'Անուն';
	description = () => 'Նկարագրություն';
	osmIntegration = () => 'OSM ինտեգրում';
	osmPull = () => 'Ներմուծել ծառեր OSM-ից';
	osmBotId = () => 'OSM բոտի օգտատիրոջ ID';
	osmBotIdHint = () => 'OSM-ից ներմուծված փոփոխությունների համար օգտագործվող տեղական օգտատիրոջ ID։';
	apply = () => 'Կիրառել';
	applying = () => 'Պահպանվում է...';
	loading = () => 'Բեռնվում է...';
	error = (description: string) => `Սխալ: ${description}`;
}

const getLocale = (): StringList => {
	return localize({
		en: new English(),
		ru: new Russian(),
		hy: new Armenian()
	});
};

export const locale = getLocale();
