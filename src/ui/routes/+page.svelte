<script>
	import { createEventDispatcher, onMount } from 'svelte';
	import { MainToolBar, Icon } from 'poro-components-ui';
	import { Editor, rootCtx, defaultValueCtx, EditorStatus } from '@milkdown/core'; // Add rootCtx here
	import { commonmark } from '@milkdown/kit/preset/commonmark';
	import { nord } from '@milkdown/theme-nord';

	const dispatch = createEventDispatcher();
	export let has_chats = false;

	let editor;
	let editorContainer;

	onMount(async () => {
		editorContainer = document.querySelector('#editor_root');
		if (editorContainer) {
			try {
				editor = Editor.make()
					.config((ctx) => {
						ctx.set(rootCtx, editorContainer); // Set the root explicitly
						ctx.set(defaultValueCtx, 'Ask Me Anything!');
					})
					.use(commonmark)
					.onStatusChange((status) => {
						if (status === EditorStatus.Created) {
							console.log('Milkdown editor fully created');
						} else if (status === EditorStatus.OnCreate) {
							console.log('Milkdown editor initializing...');
						} else if (status === EditorStatus.OnDestroy) {
							console.log('Milkdown editor destroying...');
						} else if (status === EditorStatus.Destroyed) {
							console.log('Milkdown editor destroyed');
						}
					});

				await editor.create();
				console.log('Editor container content after creation:', editorContainer.innerHTML);
			} catch (error) {
				console.error('Error initializing Milkdown editor:', error);
			}
		} else {
			console.error('Editor container not found.');
		}
	});
</script>

<div class="grid_layout">
	<div class="read_main_layout">
		{#if has_chats === true}
			<Icon
				name="poro_logo"
				class="cursor_default"
				size="12rem"
				scale="1"
				viewSize={{
					width: 221,
					height: 59
				}}
			/>
		{/if}
	</div>
	<div class="write_main_layout">
		<div class="toolbar">
			<MainToolBar />
		</div>
		<div class="editor_root" id="editor_root"></div>
	</div>
</div>

<!-- Mobile Not Supported -->
<div class="grid lg:hidden h-screen place-items-center text-white text-4xl">
	<h1>Sorry, Mobile Not Supported</h1>
</div>

<style lang="postcss">
	.grid_layout {
		@apply hidden lg:grid grid-cols-12 gap-y-2 place-items-center h-screen fixed inset-0;
	}
	.read_main_layout {
		@apply col-start-1 col-end-7 row-start-1 row-end-auto flex justify-center items-center bg-black-a w-full h-full;
	}
	.write_main_layout {
		@apply col-start-7 col-end-13 row-start-1 row-end-auto flex flex-col bg-black-b w-full h-full;
	}
	.toolbar {
		@apply fixed top-8 right-0 w-1/4 h-16 bg-black-b z-10;
	}
	.editor_root {
		@apply mt-40 w-full h-full overflow-auto bg-black-b p-4 text-white;
	}
</style>
