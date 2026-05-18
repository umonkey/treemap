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
	sendReport = () => 'Send your report';
}

class Russian extends English {
	title = (id: string) => `Сигнал ${id}`;
	tabTitle = (id: string) => `Сигнал ${id} - Деревья Еревана`;
	noDescription = () => 'Описание отсутствует';
	photoAlt = () => 'Фото сигнала';
	sendReport = () => 'Отправить свой сигнал';
}

class Armenian extends English {
	title = (id: string) => `Ահազանգ ${id}`;
	tabTitle = (id: string) => `Ահազանգ ${id} - Երևանի ծառերը`;
	noDescription = () => 'Նկարագրությունը բացակայում է';
	photoAlt = () => 'Ահազանգի լուսանկար';
	sendReport = () => 'Ուղարկել ահազանգ';
}

const getLocale = (): StringList => {
	return localize({
		en: new English(),
		ru: new Russian(),
		hy: new Armenian()
	});
};

export const locale = getLocale();
