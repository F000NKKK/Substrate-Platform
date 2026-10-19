import { registerTranslations } from "../i18n";
import { messages } from "../../host/host.generated";

const git = messages.git;

/** Source Control in Russian — what the Git backend says (its templates are generated from Rust). */
registerTranslations(
  { id: "ru", name: "Русский" },
  {
    "Source Control - Git": "Система управления версиями — Git",
    "Fetch, pull, push and other Git operations show their output here.": "Здесь отображаются выходные данные получения, извлечения, отправки и других операций Git.",
    [git.missing]: "Git не установлен или отсутствует в PATH.",
    [git.cantStart]: "Не удалось запустить git: {error}",
    [git.authFailed]: "Удаленный репозиторий запросил учетные данные, но приложение не может их запросить. Настройте помощник учетных данных Git (например, Git Credential Manager) или SSH-ключ и повторите попытку.",
    [git.noIdentity]: "Git пока не знает, кто вы — укажите имя и адрес электронной почты в разделе \"Сервис > Параметры > Система управления версиями\", затем повторите фиксацию.",
    [git.notARepository]: "Эта папка не находится в репозитории Git.",
    [git.noPushRemote]: "В этом репозитории нет удаленного репозитория для отправки.",
    [git.noOrigin]: "У этой ветви нет вышестоящей ветви и нет удаленного репозитория \"origin\" для ее отправки.",
    [git.detachedPush]: "Отсоединенный HEAD нельзя отправить — сначала извлеките ветвь.",
    [git.noUpstream]: "У текущей ветви нет вышестоящей ветви для извлечения. Сначала отправьте ее.",
    [git.nothingInProgress]: "Слияние или перемещение изменений не выполняется.",
    [git.cloneTargetNotEmpty]: "{path} уже существует и не пуста.",
    [git.notSettable]: "{key} нельзя изменить здесь.",
    [git.running]: "> git {command}",
    // `[error]` is the Output window's level tag (`defaultLineLevel`), not words: it stays.
    [git.failed]: "[error] git {command}: сбой (код выхода {code})",
    [git.killed]: "[error] git {command} остановлен сигналом",
  },
);
