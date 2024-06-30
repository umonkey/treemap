class EnglishLocale {
  public cancel(): string {
    return "Cancel";
  }

  public canopy(): string {
    return "Canopy";
  }

  public canopyLabel(): string {
    return "Canopy ⌀, m";
  }

  public circumference(): string {
    return "Circumference";
  }

  public circumferenceLabel(): string {
    return "Circumference, m";
  }

  public confirm(): string {
    return "Confirm";
  }

  public height(): string {
    return "Height";
  }

  public heightLabel(): string {
    return "Height, m";
  }

  public loading(): string {
    return "Loading...";
  }

  public loadingComments(): string {
    return "Loading comments...";
  }

  public logInToComment(): string {
    return "You need to log in to add a comment.";
  }

  public noComments(): string {
    return "No comments for this tree yet, be the first.";
  }

  public notes(): string {
    return "Notes";
  }

  public notesHint(): string {
    return "For famous trees, like: Alien Shaped Pine.";
  }

  public numberOfTrees(): string {
    return "Number of trees";
  }

  public species(): string {
    return "Species";
  }

  public speciesRecent(): string {
    return "Recent species";
  }

  public state(): string {
    return "State";
  }

  public stateHealthy(): string {
    return "Healthy";
  }

  public stateDeformed(): string {
    return "Deformed";
  }

  public stateSick(): string {
    return "Sick";
  }

  public stateDead(): string {
    return "Dead";
  }

  public stateStomp(): string {
    return "Stomp";
  }

  public stateGone(): string {
    return "Gone";
  }

  public searchTreesPlaceholder(count: number): string {
    if (count === 0) {
      return "Search trees...";
    }

    return `Search ${count} trees...`;
  }

  public formatStatusLine(state: string, updatedAt: string): string {
    return `${state.charAt(0).toUpperCase() + state.slice(1)}, checked on ${updatedAt}.`;
  }

  public logInWithGoogle(): string {
    return "Log In with Google";
  }

  public homeButton(): string {
    return "Home";
  }

  public editButton(): string {
    return "Edit";
  }

  public photoButton(): string {
    return "Photo";
  }

  public moveButton(): string {
    return "Move";
  }

  public shareButton(): string {
    return "Share";
  }

  public noData(): string {
    return "no data";
  }

  public sizeMeters(size: number): string {
    return `${size} m`;
  }

  public stateValue(value: string): string {
    return value;
  }

  public unknownSpecies(): string {
    return "Unknown species";
  }

  public speciesTitle(species: string): string {
    if (species === "Unknown" || species === "Unknown tree") {
      return this.unknownSpecies();
    }

    return species;
  }

  public commentFieldPlaceholder(): string {
    return "Add your comment...";
  }

  public sendComment(): string {
    return "Send comment";
  }

  public updateTreeDetails(): string {
    return "Update tree details";
  }

  public moveTitle(): string {
    return "Where to move the tree?";
  }

  public moveDescription(): string {
    return "Move the map to select tree correct location for the tree.";
  }

  public sharingNotSupported(): string {
    return "Your browser does not support sharing.";
  }

  public sharingError(): string {
    return "Error sharing tree.";
  }

  public shareTitle(): string {
    return "Check out this tree on the Tree Map!";
  }

  public addTitle(): string {
    return "Add new trees";
  }

  public singleTreeLabel(): string {
    return "Single tree";
  }

  public singleTreeShortLabel(): string {
    return "Single";
  }

  public treeRowLabel(): string {
    return "Row";
  }

  public newTreesHeader(): string {
    return "New Trees";
  }
}

class RussianLocale extends EnglishLocale {
  public cancel(): string {
    return "Отмена";
  }

  public canopy(): string {
    return "Диаметр кроны";
  }

  public canopyLabel(): string {
    return "Диаметр кроны, м";
  }

  public circumference(): string {
    return "Обхват ствола";
  }

  public circumferenceLabel(): string {
    return "Обхват ствола, м";
  }

  public confirm(): string {
    return "Подтвердить";
  }

  public height(): string {
    return "Высота";
  }

  public heightLabel(): string {
    return "Высота, м";
  }

  public loading(): string {
    return "Загрузка...";
  }

  public loadingComments(): string {
    return "Загружаю комментарии...";
  }

  public logInToComment(): string {
    return "Нужно залогиниться чтобы оставить комментарий.";
  }

  public noComments(): string {
    return "Нет комментариев к этому дереву, будь первым.";
  }

  public notes(): string {
    return "Примечания";
  }

  public notesHint(): string {
    return "Что-нибудь интересное, например: есть гнездо совы.";
  }

  public numberOfTrees(): string {
    return "Сколько деревьев";
  }

  public species(): string {
    return "Вид";
  }

  public speciesRecent(): string {
    return "Недавно использованные";
  }

  public state(): string {
    return "Состояние";
  }

  public stateHealthy(): string {
    return "Здоровое";
  }

  public stateDeformed(): string {
    return "Деформированное";
  }

  public stateSick(): string {
    return "Больное";
  }

  public stateDead(): string {
    return "Мёртвое";
  }

  public stateStomp(): string {
    return "Пень";
  }

  public stateGone(): string {
    return "Исчезло";
  }

  public searchTreesPlaceholder(count: number) {
    if (count === 0) {
      return "Искать деревья...";
    }

    return `Искать среди ${count} деревьев...`;
  }

  public formatStatusLine(state: string, updatedAt: string): string {
    const stateValue = this.stateValue(state);
    return `${stateValue.charAt(0).toUpperCase() + stateValue.slice(1)}, проверено ${updatedAt}.`;
  }

  public logInWithGoogle(): string {
    return "Войти через Google";
  }

  public homeButton(): string {
    return "Главная";
  }

  public editButton(): string {
    return "Править";
  }

  public photoButton(): string {
    return "Фото";
  }

  public moveButton(): string {
    return "Сдвинуть";
  }

  public shareButton(): string {
    return "Поделиться";
  }

  public noData(): string {
    return "нет данных";
  }

  public sizeMeters(size: number): string {
    return `${size} м`;
  }

  public stateValue(value: string): string {
    const values = {
      healthy: "здоровое",
      deformed: "деформированное",
      sick: "больное",
      dead: "мёртвое",
      stomp: "пень",
      gone: "исчезло",
    } as Record<string, string>;

    return values[value] || value;
  }

  public unknownSpecies(): string {
    return "Неизвестный вид";
  }

  public commentFieldPlaceholder(): string {
    return "Напиши комментарий...";
  }

  public sendComment(): string {
    return "Отправить";
  }

  public updateTreeDetails(): string {
    return "Обновление данных по дереву";
  }

  public moveTitle(): string {
    return "Куда двигать дерево?";
  }

  public moveDescription(): string {
    return "Двигай карту чтобы выбрать правильное место для дерева.";
  }

  public sharingNotSupported(): string {
    return "Твой браузер не поддерживает отправку ссылок.";
  }

  public sharingError(): string {
    return "Не удалось отправить ссылку.";
  }

  public shareTitle(): string {
    return "Глянь какое дерево!";
  }

  public addTitle(): string {
    return "Добавление деревьев";
  }

  public singleTreeLabel(): string {
    return "Одно дерево";
  }

  public singleTreeShortLabel(): string {
    return "Одно";
  }

  public treeRowLabel(): string {
    return "Ряд";
  }

  public newTreesHeader(): string {
    return "Новые деревья";
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
