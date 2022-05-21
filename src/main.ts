import App from './App.svelte';
import { emit, listen } from '@tauri-apps/api/event'

const app = new App({
	target: document.body,
	props: {
		name: 'Svelte'
	}
});

listen("backend_alert", (prop):void => {
	alert(Object(prop.payload).message);
})


export default app;