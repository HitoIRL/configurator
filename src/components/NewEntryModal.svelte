<script lang="ts">
    import Modal from "./Modal.svelte";
    import TableCells from "../icons/TableCells.svelte";
    import IconButton from "./IconButton.svelte";
    import ChatBubble from "../icons/ChatBubble.svelte";
    import Calculator from "../icons/Calculator.svelte";
    import CheckBadge from "../icons/CheckBadge.svelte";
    import { entries } from "../stores/entries";

    export let open: boolean;
    export let index: number;

    function addString() {
        entries.update((entries) => {
            const name = entries[index].type === "SequenceTable" ? `[${entries[index].value.length}]` : "newString";
            entries[index].value.push({ type: "String", name, value: "" });
            return entries;
        });
        open = false;
    }
</script>

<Modal bind:open title="Add New Entry">
    <div class="grid grid-cols-3 gap-x-2 gap-y-3">
        <IconButton text="Table">
            <TableCells class="w-9 h-9 mb-1"/>
        </IconButton>
        <IconButton on:click={addString} text="String">
            <ChatBubble class="w-9 h-9 mb-1"/>
        </IconButton>
        <IconButton text="Number">
            <Calculator class="w-9 h-9 mb-1"/>
        </IconButton>
        <IconButton text="Boolean">
            <CheckBadge class="w-9 h-9 mb-1"/>
        </IconButton>
    </div>
</Modal>
