export interface MessageDraft {
  title: string;
  body: string;
}

export interface DisposableDrafts {
  drafts: MessageDraft[];
}

export interface BackToBackDrafts {
  drafts: MessageDraft[];
}

export interface Instance {
  instance: string;
  time: string;
}

export interface SealedInstances {
  instances: Instance[];
}

export interface FinishedPlan {
  plan: Plan;
  time: string;
}

export interface FinishedPlanList {
  list: FinishedPlan;
}

export interface Plans {
  selectedPlan?: string;
  plans: Record<string, Plan>;
}

export interface Plan {
  title: string;
  body: string;
}
