<script lang="ts">
	import { onMount } from 'svelte';
	import { PUBLIC_IMG_API_URL } from '$env/static/public';

//	export let data;

	onMount(async () => {
  	console.log("+page.svelte onMount")
  	const pools = await fetch(PUBLIC_IMG_API_URL).then((ps) => ps.json());
	});
</script>

<h3>img-gallery</h3>
