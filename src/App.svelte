<script lang="ts">
	import OpenFile from "./comp/OpenFile.svelte";
	import ReadDir from "./comp/ReadDir.svelte";

	export let name: string;
	
	export let currentPage: string = "rdir";
</script>

<main style="display:flex; flex-direction: column; justify-content: center; align-items: center;">
	<h1>Hello {name}!</h1>
	<section style="position:sticky; padding:20pt 0pt; margin: 0pt 0pt 20pt 0pt; top:0px; background-color: hsla(0,0%,100%, 20%); -webkit-backdrop-filter: blur(20pt); width:100vw; display:flex; justify-content: center; align-items: center; border-bottom: 1px solid hsl(0,0%,80%);">
		<section style="display: flex">
			<label>
				<input type=radio bind:group={currentPage} name="currentPage" value={"rdir"}/>
				Read Directory
			</label>
	
			<label>
				<input type=radio bind:group={currentPage} name="currentPage" value={"ofile"}/>
				Open File
			</label>
		</section>
	</section>

	<!-- read dir component -->
	{#if currentPage === "rdir"}
		<ReadDir />
	{:else if currentPage === "ofile"}
		<OpenFile />
	{/if}
</main>

<style>
	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	h1 {
		color: #ff3e00;
		text-transform: uppercase;
		font-size: 4em;
		font-weight: 100;
	}

	label{
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;

		margin: 0 10pt 0 10pt;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>