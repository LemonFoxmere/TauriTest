<script lang="ts">
	import { FileEntry, readDir } from "@tauri-apps/api/fs";
    export let currentDir:Promise<FileEntry[]> = null;
    export let inputDir:string = "./";
    export let openFile = ():void => {
		currentDir = readDir(inputDir);
	};
</script>

<h2 style="margin:0; line-height: 150%;">Pick a directory</h2>
<p style="margin:0; color:hsl(0,0%,45%)">And I'll try to show its content...</p>

<hr style="width: 50pt; height:1px; border:none; background-color: hsl(0,0%,80%); margin:10pt 0 10pt 0">

<input type="text" bind:value={inputDir}>

<section style="display:flex">
    <button on:click={openFile} style="display: flex; justify-content: center; align-items: center;">What's in <code style="background-color: hsl(200,5%,35%); color:hsl(0,0%,100%); padding: 1pt 5pt 1pt 5pt; margin:0px 5pt 0 5pt; border-radius: 3pt;">{inputDir}</code> ?</button>
    {#if currentDir}
        <button style="margin-left:10pt" on:click={() => currentDir=null}>Clear</button>
    {/if}
</section>

{#if !currentDir}
    <p>Nothing here at the moment...</p>
{:else}
    {#await currentDir}
        <p>Reading Directory...</p>
    {:then resp}
        {#each resp as paths}
            <p style="margin: 0 0 5pt 0">{paths.name}</p>
            <p style="margin: 0 0 20pt 0"><span style="font-family: monospace; color:hsl(200,50%,40%)">{paths.path}</span></p>
        {/each}
    {:catch error}
        <p>Yikes... I cannot open the file.</p>
        <p style="font-size:1rem; width:min(500pt, 90vw); font-family: monospace;">Reason:<br><span style="color:hsl(0,80%,40%)">{error}</span></p>
    {/await}
{/if}