import { Instance, Plan, PlanDrafts, SealedInstances } from "@/types";
import { invoke } from "@tauri-apps/api/core";
import { error } from "@tauri-apps/plugin-log";

export async function testCommnication() {
  try {
    await invoke("set_disposable_msg", { msg: "Hello World!" });
  } catch (e: unknown) {
    error(String(e));
    alert(e);
  }
}

function randString() {
  const chars =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
  let result = "";

  for (let i = 0; i < 15; i++) {
    const randomIndex = Math.floor(Math.random() * chars.length);
    result += chars.charAt(randomIndex);
  }

  return result;
}

export async function genRandomSeal() {
  const sealed: SealedInstances = { instances: [] };
  for (let i = 0; i < 5; i++) {
    const rand: Instance = {
      time: "2077-4-1 00:00:00",
      instance: randString(),
    };
    sealed.instances.push(rand);
  }
  await invoke("store_sealed_instances", { data: sealed });
}

export async function genRandomPlan() {
  const plans: PlanDrafts = { drafts: new Map() };
  for (let i = 0; i < 5; i++) {
    const uuid = crypto.randomUUID();
    const rand: Plan = {
      title: randString(),
      body: randString() + randString(),
    };
    plans.drafts.set(uuid, rand);
  }
  try {
    await invoke("store_plan_drafts", { data: plans });
  } catch (e) {
    alert(e);
  }
}

export function getTimeStamp() {
  const d = new Date();
  const timestamp = d.toLocaleDateString() + " " + d.toLocaleTimeString();
  return timestamp;
}

export function randomUUID(): string {
  return crypto.randomUUID();
}
