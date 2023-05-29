import { Updatable } from '/elements/updatable.js';

export class UpdatableExample extends Updatable {
    constructor() {
        super();
        const title = this.getAttribute('data-title') || ''; // Get the value of data-title attribute
        this.shadowRoot.innerHTML = `<label>${title}: <span id="text"></span></label>`;
    }
    update(value) {
        const span = this.shadowRoot.getElementById("text");
        span.textContent = value+ ";";
    }
}
customElements.define('updatable-element', UpdatableExample);