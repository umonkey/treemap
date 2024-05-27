class EnglishLocale {
  public cancel() {
    return "Cancel";
  }

  public canopy() {
    return "Canopy ⌀, m";
  }

  public circumference() {
    return "Circumference, m";
  }

  public confirm() {
    return "Confirm";
  }

  public height() {
    return "Height, m";
  }

  public notes() {
    return "Notes";
  }

  public notesHint() {
    return "For famous trees, like: Alien Shaped Pine.";
  }

  public species() {
    return "Species";
  }

  public speciesRecent() {
    return "Recent species";
  }

  public state() {
    return "State";
  }

  public stateHealthy() {
    return "Healthy";
  }

  public stateDeformed() {
    return "Deformed";
  }

  public stateSick() {
    return "Sick";
  }

  public stateDead() {
    return "Dead";
  }

  public stateStomp() {
    return "Stomp";
  }

  public stateGone() {
    return "Gone";
  }
}

class RussianLocale extends EnglishLocale {
  public cancel() {
    return "Отмена";
  }

  public canopy() {
    return "Диаметр кроны, м";
  }

  public circumference() {
    return "Обхват ствола, м";
  }

  public confirm() {
    return "Подтвердить";
  }

  public height() {
    return "Высота, м";
  }

  public notes() {
    return "Примечания";
  }

  public notesHint() {
    return "Что-нибудь интересное, например: есть гнездо совы.";
  }

  public species() {
    return "Вид";
  }

  public speciesRecent() {
    return "Недавно использованные";
  }

  public state() {
    return "Состояние";
  }

  public stateHealthy() {
    return "Здоровое";
  }

  public stateDeformed() {
    return "Деформированное";
  }

  public stateSick() {
    return "Больное";
  }

  public stateDead() {
    return "Мёртвое";
  }

  public stateStomp() {
    return "Пень";
  }

  public stateGone() {
    return "Исчезло";
  }
}

const getLocale = () => {
  const lang = navigator.language || "en-US";
  console.debug(`[locale] Language=${lang}`);

  if (lang === "ru-RU") {
    console.debug("[locale] Using Russian locale");
    return new RussianLocale();
  }

  return new EnglishLocale();
};

export const locale = getLocale();
