<script lang="ts">
    import Checkbox from "./Checkbox.svelte";
    import TextInput from "./TextInput.svelte";
    import IntInput from "./IntInput.svelte";
    import { formatName } from "../helpers";
    import Plus from "../icons/Plus.svelte";
    import NewEntryModal from "./NewEntryModal.svelte";
    import type { Entry } from "../stores/entries";
    
    export let entries: Entry[] = [];
    export let index: number | undefined = undefined; // !undefined = inner table

    let entryModal = false;

    console.log(entries);
</script>

{#if entries.length > 0}
    <div class="flex flex-col p-4 gap-3 {index !== undefined ? "border border-neutral-500" : ""}">
        {#each entries as entry, index}
            {#if entry.type === "Bool"}
                <Checkbox name={entry.name} value={entry.value}/>
            {:else if entry.type === "String"}
                <TextInput name={entry.name} value={entry.value}/>
            {:else if entry.type === "Int"}
                <IntInput name={entry.name} value={entry.value}/>
            {:else if entry.type === "Table" || entry.type === "SequenceTable"}
                <div>
                    <p class="mt-3">{formatName(entry.name)}</p>
                    <svelte:self entries={entry.value} {index}/>
                </div>
            {/if}
        {/each}
        {#if index !== undefined}
            <NewEntryModal bind:open={entryModal} {index}/>
            <button on:click={() => entryModal = true} class="py-1 bg-neutral-700 flex justify-center rounded transition-colors hover:bg-neutral-700/60">
                <Plus class="h-9 w-9"/>
            </button>
        {/if}
    </div>
{/if}
