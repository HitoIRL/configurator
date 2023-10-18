<script lang="ts">
    import XMark from "../icons/XMark.svelte";

    export let open: boolean;
    export let title: string;

    let clazz = "";
    export { clazz as class };
</script>

{#if open}
    <div class="left-0 top-0 fixed w-screen h-screen bg-black/40"></div>
    <div class="{clazz} fixed left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 bg-neutral-800 p-10 rounded-lg flex flex-col">
        <button on:click={() => open = false} class="absolute right-3 top-4">
            <XMark class="h-7 w-7"/>
        </button>
        <h1 class="text-center text-2xl font-bold mb-5">{title}</h1>
        <slot/>
    </div>
{/if}
