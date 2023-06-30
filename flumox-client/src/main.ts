import App from './components/App.svelte';
import "./styles/main.css";

const app = new App({
  target: document.getElementById('app'),
});

if ('serviceWorker' in navigator) {
  navigator.serviceWorker.register('/worker.js', { scope: '/' });
}
