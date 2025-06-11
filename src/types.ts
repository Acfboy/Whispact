export interface MessageDraft {
  title: string;
  body: string;
}

export interface DisposableDrafts {
  drafts: MessageDraft[];
}

export interface Instance {
  instance: string;
  time: string;
}

export interface SealedInstances {
  instances: Instance[];
}

export interface PlanDrafts {
  drafts: Map<string, Plan>;
}

export interface FinishedPlan {
  plan: Plan;
  time: string;
}

export interface FinishedPlanList {
  list: FinishedPlan[];
}

export interface SyncPlans {
  selectedPlan?: string;
  plans: Map<string, Plan>;
}

export interface Plan {
  title: string;
  body: string;
}

export enum MessageType {
  Disposable,
  PlanSync,
  Empty,
  Seal,
}

export interface Mail {
  cover: string;
  inner: MailInner;
}

export interface MailInner {
  title: string;
  body: string;
}

export interface MailCover {
  sealed: boolean;
  cover: string;
}

export interface MailCoverList {
  mails: Map<string, MailCover>;
}
