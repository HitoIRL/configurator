<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { file } from "../stores/file";
    import ArrowLongLeft from "../icons/ArrowLongLeft.svelte";
    import CloudArrowDown from "../icons/CloudArrowDown.svelte";
  import Table from "./Table.svelte";

    type Entry = {
        type: string;
        name: string;
        value: any;
    }

    function walkTable(table: any[]) {
        const entries: Entry[] = [];

        table.forEach(e => {
            const type = Object.keys(e)[0];
            const [key, value] = e[type];

            if (type === "Table") {
                const subEntries = walkTable(value);
                entries.push({ type, name: key, value: subEntries })
            } else {
                entries.push({ type, name: key, value })
            }
        });

        return entries;
    }

    let entries: Entry[];
    if ($file) {
        const reader = new FileReader();
        reader.onload = async (e) => {
            const contents = e.target?.result as string;
            let interpreted: any = await invoke("interpret_contents", { contents });
            entries = walkTable(interpreted);
        };
        reader.readAsText($file);
    }
</script>

<div class="flex px-4 py-1 bg-violet-700">
    <button on:click={() => $file = null}>
        <ArrowLongLeft class="w-8 h-8"/>
    </button>
    <button class="ml-auto">
        <CloudArrowDown class="w-8 h-8"/>
    </button>
    <!-- TODO: read fxmanifest and display informations in header such as script name, version, etc -->
</div>
<div class="flex flex-col gap-4 p-3">
    {#if entries}
        <Table entries={entries}/>
    {/if}
</div>
