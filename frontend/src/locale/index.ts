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

  public loading() {
    return "Loading...";
  }

  public loadingComments() {
    return "Loading comments...";
  }

  public logInToComment() {
    return "You need to log in to add a comment.";
  }

  public noComments() {
    return "No comments for this tree yet, be the first.";
  }

  public notes() {
    return "Notes";
  }

  public notesHint() {
    return "For famous trees, like: Alien Shaped Pine.";
  }

  public numberOfTrees() {
    return "Number of trees";
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

  public loading() {
    return "Загрузка...";
  }

  public loadingComments() {
    return "Загружаю комментарии...";
  }

  public logInToComment() {
    return "Нужно залогиниться чтобы оставить комментарий.";
  }

  public noComments() {
    return "Нет комментариев к этому дереву, будь первым.";
  }

  public notes() {
    return "Примечания";
  }

  public notesHint() {
    return "Что-нибудь интересное, например: есть гнездо совы.";
  }

  public numberOfTrees() {
    return "Сколько деревьев";
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
