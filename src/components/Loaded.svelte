<script lang="ts">
    import { file } from "../stores/file";
    import { invoke } from "@tauri-apps/api/tauri";
    import ArrowLongLeft from "../icons/ArrowLongLeft.svelte";

    function walkTable(table: any[]) {
        const entries: any[] = [];

        table.forEach(e => {
            const type = Object.keys(e)[0];
            const [key, value] = e[type];

            if (type === "Table") {
                const subEntries = walkTable(value);
                entries.push({ type, key, value: subEntries })
            } else {
                entries.push({ type, key, value })
            }
        });

        return entries;
    }

    let entries: any;
    if ($file) {
        const reader = new FileReader();
        reader.onload = async (e) => {
            const contents = e.target?.result as string;
            entries = await invoke("interpret_contents", { contents });
        };
        reader.readAsText($file);
    }

    $: if (entries) {
        console.log(walkTable(entries))
    }
</script>

<div class="px-3 py-1 bg-violet-700">
    <button on:click={() => $file = null} class="flex cursor-pointer">
        <ArrowLongLeft class="w-8 h-8"/>
    </button>
    <!-- TODO: read fxmanifest and display informations in header such as script name, version, etc -->
</div>
