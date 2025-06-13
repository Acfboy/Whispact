import {
  Instance,
  MailCover,
  MailCoverList,
  MailInner,
  Plan,
  PlanDrafts,
  SealedInstances,
} from "@/types";
import { invoke, InvokeArgs } from "@tauri-apps/api/core";
import { emit } from "@tauri-apps/api/event";
import { error } from "@tauri-apps/plugin-log";

export async function testCommnication() {
  try {
    await try_invoke("set_disposable_msg", { msg: "Hello World!" });
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
  await try_invoke("store_sealed_instances", { data: sealed });
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
  await try_invoke("store_plan_drafts", { data: plans });
}

export function getTimeStamp() {
  const d = new Date();
  const timestamp = d.toLocaleDateString() + " " + d.toLocaleTimeString();
  return timestamp;
}

export function randomUUID(): string {
  return crypto.randomUUID();
}

export async function genRandomMail() {
  const uuid = randomUUID();
  const cover: MailCover = {
    sealed: false,
    cover: randString(),
    timestamp: getTimeStamp(),
  };
  const coverList: MailCoverList = { mails: new Map([[uuid, cover]]) };
  const inner: MailInner = {
    title: randString(),
    body: randString() + randString(),
  };
  await try_invoke("store_mail_drafts_covers", { data: coverList });
  await try_invoke("store_mail_inner", { uuid, data: inner });
}

export async function genRandomInbox() {
  const coverList: MailCoverList = { mails: new Map([]) };
  for (let i = 0; i < 5; i++) {
    const uuid = randomUUID();
    const cover: MailCover = {
      sealed: true,
      cover: randString(),
      timestamp: getTimeStamp(),
    };
    const inner: MailInner = {
      title: randString(),
      body: randString() + randString(),
    };
    await try_invoke("store_mail_inner", { uuid, data: inner });
    coverList.mails.set(uuid, cover);
  }
  await try_invoke("store_mail_covers", { data: coverList });
}

export async function try_invoke<T>(
  command: string,
  argv?: InvokeArgs,
): Promise<T | undefined> {
  try {
    return await invoke<T>(command, argv);
  } catch (e) {
    emit("err", e);
    return undefined;
  }
}

export function timeStampUuid(): string {
  const uuidTime = BigInt(Date.now());

  const timeLow = Number(uuidTime & BigInt(0xffff));
  const timeHigh = Number((uuidTime >> BigInt(16)) & BigInt(0xffff_ffff));

  const rand1 = Math.floor(Math.random() * 0xfff);
  const rand2 = Math.floor(Math.random() * 0xffff);
  const rand3 = Math.floor(Math.random() * 0xffff_ffff_ffff);

  return `${timeHigh.toString(16).padStart(8, "0")}-${timeLow
    .toString(16)
    .padStart(4, "0")}-1${rand1.toString(16).padStart(3, "0")}-${rand2
    .toString(16)
    .padStart(4, "0")}-${rand3.toString(16).padStart(12, "0")}`;
}
