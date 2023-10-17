<script lang="ts">
    import Checkbox from "./Checkbox.svelte";
    import TextInput from "./TextInput.svelte";
    import IntInput from "./IntInput.svelte";
    import { formatName } from "../helpers";
    import Plus from "../icons/Plus.svelte";
    
    export let entries: any[];
    export let inner = false;
</script>

{#if entries.length > 0}
    <div class="flex flex-col p-4 gap-3 {inner ? "border border-neutral-500" : ""}">
        {#each entries as entry}
            {#if entry.type === "Bool"}
                <Checkbox name={entry.name} value={entry.value}/>
            {:else if entry.type === "String"}
                <TextInput name={entry.name} value={entry.value}/>
            {:else if entry.type === "Int"}
                <IntInput name={entry.name} value={entry.value}/>
            {:else if entry.type === "Table"}
                <div>
                    <p class="mt-3">{formatName(entry.name)}</p>
                    <svelte:self entries={entry.value} inner={true}/>
                </div>
            {/if}
        {/each}
        {#if inner}
            <button class="py-1 bg-neutral-700 flex justify-center rounded transition-colors hover:bg-neutral-700/60">
                <Plus class="h-9 w-9"/>
            </button>
        {/if}
    </div>
{/if}
