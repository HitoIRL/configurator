import { writable } from "svelte/store";

export type Entry = {
    type: "String" | "Int" | "Float" | "Bool" | "Table" | "SequenceTable";
    name: string;
    value: any;
}

export const entries = writable<Entry[]>([]);
