import { RichText } from './index';
import './style.css';
import page from '../../../assets/output/index.html?raw';


// Define the custom element if it's not already defined
if (!customElements.get('rich-text')) {
  customElements.define('rich-text', RichText);
}

const root = document.querySelector<HTMLDivElement>('#app')!;


root.innerHTML = page;
