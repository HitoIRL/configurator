<script lang="ts">
    import Checkbox from "./Checkbox.svelte";
    import TextInput from "./TextInput.svelte";
    import IntInput from "./IntInput.svelte";
    import { formatName } from "../helpers";
    
    export let entries: any[];
</script>

{#if entries.length > 0}
    <div class="flex flex-col border border-neutral-500 p-4 gap-3">
        {#each entries as entry}
            {#if entry.type === "Bool"}
                <Checkbox name={entry.name} value={entry.value}/>
            {:else if entry.type === "String"}
                <TextInput name={entry.name} value={entry.value}/>
            {:else if entry.type === "Int"}
                <IntInput name={entry.name} value={entry.value}/>
            {:else if entry.type === "Table"}
                <p class="mt-3">{formatName(entry.name)}</p>
                <svelte:self entries={entry.value}/>
            {/if}
        {/each}
    </div>
{/if}
