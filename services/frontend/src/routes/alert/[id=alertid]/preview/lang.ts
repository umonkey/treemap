import { localize } from '$lib/locale';

interface StringList {
	title: (id: string) => string;
	tabTitle: (id: string) => string;
	noDescription: () => string;
	photoAlt: () => string;
	sendReport: () => string;
}

class English implements StringList {
	title = (id: string) => `Alert ${id}`;
	tabTitle = (id: string) => `Alert ${id} - Trees of Yerevan`;
	noDescription = () => 'No description provided';
	photoAlt = () => 'Alert photo';
	sendReport = () => 'Sent via @kanach_yerevan_bot';
}

class Russian extends English {
	title = (id: string) => `Сигнал ${id}`;
	tabTitle = (id: string) => `Сигнал ${id} - Деревья Еревана`;
	noDescription = () => 'Описание отсутствует';
	photoAlt = () => 'Фото сигнала';
	sendReport = () => 'Отправлено через @kanach_yerevan_bot';
}

class Armenian extends English {
	title = (id: string) => `Ահազանգ ${id}`;
	tabTitle = (id: string) => `Ահազանգ ${id} - Երևանի ծառերը`;
	noDescription = () => 'Նկարագրությունը բացակայում է';
	photoAlt = () => 'Ահազանգի լուսանկար';
	sendReport = () => 'Ուղարկված է @kanach_yerevan_bot-ի միջոցով';
}

const getLocale = (): StringList => {
	return localize({
		en: new English(),
		ru: new Russian(),
		hy: new Armenian()
	});
};

export const locale = getLocale();
