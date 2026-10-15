export {
  useDocumentHistory,
  documentHistory,
  recordDocumentChange,
  pushDocumentStep,
  documentAction,
  setDocumentWriter,
  forgetDocumentHistory,
  undoable,
  undoScopeOf,
  SETTINGS_HISTORY,
} from "./history";
export type { DocumentHistory, DocumentStep } from "./history";
export { UndoScope } from "./UndoScope";
