<script lang="ts">
	import { Marquee } from '@repo/ui';
	import { onMount } from 'svelte';

	let { data } = $props();
	const { title } = data;
	let dir = $state<'ver' | 'hor' | null>(null);

	onMount(() => {
		setTimeout(
			() => {
				window.location.reload();
			},
			60 * 1000 * 60
		);
		const { innerWidth: width, innerHeight: height } = window;
		dir = width > height ? 'hor' : 'ver';
	});
</script>

<svelte:head>
	<title>News</title>
</svelte:head>

{#if dir}
	<div
		class="full-screen center-content font-dot-gothic whitespace-nowrap font-bold tracking-widest text-black {dir}"
		style:background-color="hsl({+(Math.random() * 200)}, 100%, 80%)"
	>
		<Marquee text={title} timeout={700} />
	</div>
{/if}

<style lang="postcss">
	.ver {
		writing-mode: vertical-rl;
		text-orientation: mixed;
		font-size: 70vw;
	}

	.hor {
		font-size: 70vh;
	}
</style>
