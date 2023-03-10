import { writable } from "svelte/store";
import type { IPlayer } from "./types/server";

export const sessionID = writable<null | string>(null);

export const userdata = writable<IPlayer | null>(null);