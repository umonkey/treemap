import { localize } from '$lib/locale';

interface StringList {
	title: () => string;
	heading: () => string;
	intro: () => string;
	readOnly: () => string;
	coverage: () => string;
	endpoint: () => string;
	documentation: () => string;
}

class English implements StringList {
	title = () => 'Remote MCP Server';
	heading = () => 'Remote MCP Server';
	intro = () =>
		'This application runs a remote Model Context Protocol (MCP) server that lets AI agents, such as Gemini, query the tree database directly.';
	readOnly = () =>
		'The server is read-only: no authentication is required and no update operations are implemented, so agents can read data but never change it.';
	coverage = () =>
		'Tool coverage is basic but useful — agents can find the tallest and widest trees, street statistics, citizen alerts, and addresses.';
	endpoint = () => 'Endpoint';
	documentation = () => 'Read the full documentation';
}

class Russian extends English {
	title = () => 'Удалённый MCP-сервер';
	heading = () => 'Удалённый MCP-сервер';
	intro = () =>
		'Приложение предоставляет удалённый сервер Model Context Protocol (MCP), который позволяет ИИ-агентам, например Gemini, напрямую обращаться к базе данных деревьев.';
	readOnly = () =>
		'Сервер работает только для чтения: аутентификация не требуется, а операции обновления не реализованы, поэтому агенты могут читать данные, но не изменять их.';
	coverage = () =>
		'Набор инструментов базовый, но полезный: агенты могут находить самые высокие и самые широкие деревья, статистику по улицам, обращения жителей и адреса.';
	endpoint = () => 'Адрес';
	documentation = () => 'Полная документация';
}

class Armenian extends English {
	title = () => 'Հեռավոր MCP սերվեր';
	heading = () => 'Հեռավոր MCP սերվեր';
	intro = () =>
		'Հավելվածը տրամադրում է հեռավոր Model Context Protocol (MCP) սերվեր, որը թույլ է տալիս ԱԲ գործակալներին, օրինակ՝ Gemini-ին, ուղղակիորեն հարցումներ կատարել ծառերի տվյալների բազայից։';
	readOnly = () =>
		'Սերվերը միայն ընթերցման համար է. նույնականացում չի պահանջվում, իսկ թարմացման գործողություններ չեն իրականացվում, ուստի գործակալները կարող են կարդալ տվյալները, բայց չեն կարող փոփոխել դրանք։';
	coverage = () =>
		'Գործիքների ծածկույթը հիմնական է, բայց օգտակար. գործակալները կարող են գտնել ամենաբարձր և ամենալայն ծառերը, փողոցների վիճակագրությունը, քաղաքացիների ծանուցումները և հասցեները։';
	endpoint = () => 'Հասցե';
	documentation = () => 'Ամբողջական փաստաթղթեր';
}

const getLocale = (): StringList => {
	return localize({
		en: new English(),
		ru: new Russian(),
		hy: new Armenian()
	});
};

export const locale = getLocale();
