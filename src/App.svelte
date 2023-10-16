<script lang="ts">
    import Loaded from "./components/Loaded.svelte";
    import NotLoaded from "./components/NotLoaded.svelte";
    import { file } from "./stores/file";
</script>

<div class="flex flex-col">
    {#if $file}
        <Loaded/>
    {:else}
        <NotLoaded/>
    {/if}
</div>

<style lang="postcss">
    @import url("https://fonts.googleapis.com/css2?family=Mitr:wght@400;700&display=swap");

    :global(body) {
        background-color: theme(colors.neutral.900);
        color: theme(colors.white);
        font-family: "Mitr", sans-serif;
    }
</style>
