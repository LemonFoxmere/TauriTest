<script lang="ts">
    export let currentFile:FileList = null;
</script>

<h2 style="margin:0; line-height: 150%;">Pick a file</h2>
<p style="margin:0; color:hsl(0,0%,45%)">And I'll try to show its content...</p>

<hr style="width: 50pt; height:1px; border:none; background-color: hsl(0,0%,80%); margin:10pt 0 10pt 0">

<button on:click={() => document.getElementById('file-input').click()}>Open a file</button>
<input bind:files={currentFile} id="file-input" type="file" accept="text/txt" style="display:none">

{#if !!currentFile}
    <p>File name: <strong>{currentFile[0].name}</strong></p>

    {#await currentFile[0].text()}
        <p>Reading file content...</p>
    {:then dat}
        <p style="width:min(400pt, 90vw)">Content: {dat}</p>
    {:catch error}
        <p>Yikes... I cannot read the file.</p>
        <p style="font-size:1rem; width:min(500pt, 90vw); font-family: monospace;">Reason:<br><span style="color:hsl(0,80%,40%)">{error}</span></p>
    {/await}
{/if}