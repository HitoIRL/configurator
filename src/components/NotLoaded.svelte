<script lang="ts">
    import { file } from "../stores/file";
    import ArrowRightCircle from "../icons/ArrowRightCircle.svelte";

    function formatBytes(bytes: number, decimals = 2) {
        if (!+bytes) return "0 B"

        const k = 1024
        const dm = decimals < 0 ? 0 : decimals
        const sizes = ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB", "YiB"]

        const i = Math.floor(Math.log(bytes) / Math.log(k))
        return `${parseFloat((bytes / Math.pow(k, i)).toFixed(dm))} ${sizes[i]}`
    }

    let tempFile: File | null = null;
    
    function fileUpload(event: Event) {
        const target = event.target as HTMLInputElement;
        if (target.files?.length && target.files.length > 0) {
            tempFile = target.files[0];
        }
    }

    async function readFile() {
        $file = tempFile;
    }
</script>

<div class="h-screen flex flex-col items-center justify-center">
    <h1 class="text-5xl font-bold mb-2">Configurator</h1>
    <p class="text-xl">Looks like you didn't load any config file yet!</p>
    <label class="cursor-pointer mt-8 bg-violet-700 px-7 py-4 rounded-xl transition-colors hover:bg-violet-800">
        <input on:change={fileUpload} class="hidden" accept=".lua" type="file" name="file">
        Select File
    </label>
    {#if tempFile !== null}
        <p class="text-lg text-neutral-400 mt-2">Selected file: {tempFile.name} ({formatBytes(tempFile.size)})</p>
        <button on:click={readFile} class="absolute bottom-0 mb-5 fill-violet-700 transition-colors hover:fill-violet-800">
            <!-- TODO: set the inside of the arrow to white // idk how XD -->
            <ArrowRightCircle class="w-16 h-16"/>           
        </button>
    {:else}
        <p class="text-lg text-neutral-400 mt-2">...</p>
    {/if}
</div>
